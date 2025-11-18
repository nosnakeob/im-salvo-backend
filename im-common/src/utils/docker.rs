use anyhow::Result;
use std::process::Command;
use std::time::Duration;
use tracing::{error, info, warn};

/// Docker 服务管理器
///
/// 创建时自动启动 Docker Desktop 和 docker-compose 服务
/// 析构时自动停止 docker-compose 服务
pub struct DockerManager {
    /// 是否由本实例启动的 Docker
    started_docker: bool,
    /// 是否由本实例启动的 compose 服务
    started_compose: bool,
}

impl DockerManager {
    /// 创建 Docker 管理器并启动所有服务
    pub async fn new() -> Result<Self> {
        let mut manager = DockerManager {
            started_docker: false,
            started_compose: false,
        };

        manager.ensure_docker_running().await?;
        manager.ensure_compose_running().await?;

        Ok(manager)
    }

    /// 确保 Docker Desktop 正在运行
    async fn ensure_docker_running(&mut self) -> Result<()> {
        if Self::is_docker_running() {
            info!("Docker 已在运行");
            return Ok(());
        }

        info!("检测到 Docker 未运行，正在启动 Docker Desktop...");
        self.start_docker_desktop().await?;
        self.started_docker = true;

        Ok(())
    }

    /// 确保 docker-compose 服务正在运行
    async fn ensure_compose_running(&mut self) -> Result<()> {
        if Self::is_compose_running() {
            info!("docker-compose 服务已在运行");
            return Ok(());
        }

        info!("检测到 docker-compose 服务未运行，正在启动...");
        self.start_compose().await?;
        self.started_compose = true;

        Ok(())
    }

    /// 检查 Docker 是否运行
    fn is_docker_running() -> bool {
        Command::new("docker")
            .arg("info")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// 检查 docker-compose 服务是否运行
    fn is_compose_running() -> bool {
        let output = Command::new("docker-compose").arg("ps").output();

        if let Ok(output) = output {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // 检查 postgres 和 redis 是否都在运行
            output_str.contains("postgres") && output_str.contains("redis")
        } else {
            false
        }
    }

    /// 启动 Docker Desktop
    async fn start_docker_desktop(&self) -> Result<()> {
        let docker_path = "C:\\Program Files\\Docker\\Docker\\Docker Desktop.exe";

        Command::new(docker_path)
            .spawn()
            .map_err(|e| anyhow::anyhow!("无法启动 Docker Desktop: {}", e))?;

        // 等待 Docker 守护进程就绪
        let max_wait = 60; // 最多等待 60 次，每次 2 秒
        for i in 0..max_wait {
            tokio::time::sleep(Duration::from_secs(2)).await;

            if Self::is_docker_running() {
                info!("Docker Desktop 启动成功！");
                return Ok(());
            }

            if i % 5 == 0 && i > 0 {
                info!("等待 Docker 启动... ({}/{}秒)", i * 2, max_wait * 2);
            }
        }

        anyhow::bail!("Docker Desktop 启动超时")
    }

    /// 启动 docker-compose 服务
    async fn start_compose(&self) -> Result<()> {
        let output = Command::new("docker-compose")
            .arg("up")
            .arg("-d")
            .output()
            .map_err(|e| anyhow::anyhow!("无法执行 docker-compose: {}", e))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("docker-compose 启动失败: {}", error_msg);
        }

        info!("docker-compose 服务启动成功！");

        // 等待服务完全启动
        info!("等待数据库服务就绪...");
        tokio::time::sleep(Duration::from_secs(5)).await;

        Ok(())
    }

    /// 停止 docker-compose 服务
    fn stop_compose(&self) -> Result<()> {
        info!("正在停止 docker-compose 服务...");

        let output = Command::new("docker-compose")
            .arg("stop")
            .output()
            .map_err(|e| anyhow::anyhow!("无法执行 docker-compose stop: {}", e))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            warn!("docker-compose 停止失败: {}", error_msg);
            return Err(anyhow::anyhow!("docker-compose 停止失败"));
        }

        info!("docker-compose 服务已停止");
        Ok(())
    }

    /// 手动停止服务（可选）
    pub fn shutdown(&self) -> Result<()> {
        if self.started_compose {
            self.stop_compose()?;
        }
        Ok(())
    }
}

impl Drop for DockerManager {
    fn drop(&mut self) {
        // 只停止由本实例启动的服务
        if self.started_compose {
            if let Err(e) = self.stop_compose() {
                error!("停止 docker-compose 服务失败: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_docker_manager() {
        // 创建管理器，自动启动服务
        let manager = DockerManager::new().await.unwrap();

        // 验证服务正在运行
        assert!(DockerManager::is_docker_running());
        assert!(DockerManager::is_compose_running());

        // 管理器离开作用域时会自动停止服务
        drop(manager);
    }
}

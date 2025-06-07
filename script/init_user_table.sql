-- 初始化User表的SQL脚本

-- 如果表已存在则删除
DROP TABLE IF EXISTS "users";

-- 创建用户表
CREATE TABLE "users"
(
    id       SERIAL PRIMARY KEY,
    username VARCHAR(50)  NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL
);

-- 添加5条示例数据
INSERT INTO "users" (username, password)
VALUES ('admin', '0192023a7bbd73250516f069df18b500'),     -- 密码: admin123
       ('user1', '7c6a180b36896a0a8c02787eeafb0e4c'),     -- 密码: password1
       ('user2', '6cb75f652a9b52798eb6cf2201057c73'),     -- 密码: password2
       ('developer', '017e0c498e2978a3ce9e58598bc116a6'), -- 密码: dev12345
       ('tester', 'c06db68e819be6ec3d26c6038d8e8d1f') -- 密码: test12345
;

-- 添加注释
COMMENT
ON TABLE "users" IS '用户表';
COMMENT
ON COLUMN "users".id IS '用户ID';
COMMENT
ON COLUMN "users".username IS '用户名';
COMMENT
ON COLUMN "users".password IS '密码(加密存储)';
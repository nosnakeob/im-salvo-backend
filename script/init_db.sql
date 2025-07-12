-- 清理现有结构以便于重跑脚本 (可选，开发时常用)
DROP TABLE IF EXISTS "friendship", "file", "message", "conversation_participant", "conversation", "users";
DROP TYPE IF EXISTS "conv_type", "msg_type", "user_status", "participant_role", "friend_status";

-- 1. 定义枚举类型
-- 这些类型提供了数据约束，确保字段值在预定义集合内。

CREATE TYPE "user_status" AS ENUM ('online', 'offline');
CREATE TYPE "conv_type" AS ENUM ('one_on_one', 'group', 'ai_chat');
CREATE TYPE "msg_type" AS ENUM ('text', 'image', 'file', 'system');
CREATE TYPE "participant_role" AS ENUM ('admin', 'member');
CREATE TYPE "friend_status" AS ENUM ('pending', 'accepted', 'blocked');

-- 2. 创建核心实体表

-- USER (用户表)
CREATE TABLE "users" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "username" VARCHAR(50) UNIQUE NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    "status" user_status DEFAULT 'offline',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- CONVERSATION (会话表)
CREATE TABLE "conversation" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "type" conv_type NOT NULL,
    "name" VARCHAR(100), -- 对于群聊是群名
    "creator_id" UUID REFERENCES "users"("id") ON DELETE SET NULL, -- 创建者，可为空
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- MESSAGE (消息表)
CREATE TABLE "message" (
    "id" BIGSERIAL PRIMARY KEY, -- 使用 BIGSERIAL 更适合高频写入的消息表
    "conversation_id" UUID NOT NULL REFERENCES "conversation"("id") ON DELETE CASCADE,
    "sender_id" UUID REFERENCES "users"("id") ON DELETE SET NULL, -- 发送者注销后消息依然保留
    "type" msg_type NOT NULL,
    "content" JSONB NOT NULL, -- 使用 JSONB 效率更高
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- FILE (文件元数据表)
CREATE TABLE "file" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "uploader_id" UUID NOT NULL REFERENCES "users"("id") ON DELETE CASCADE,
    "storage_key" VARCHAR(255) UNIQUE NOT NULL, -- S3 key or file path
    "public_url" VARCHAR(255) NOT NULL,
    "filename" VARCHAR(255) NOT NULL,
    "mime_type" VARCHAR(100) NOT NULL,
    "size_bytes" BIGINT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 3. 创建关系表

-- CONVERSATION_PARTICIPANT (会话参与者，连接表)
CREATE TABLE "conversation_participant" (
    "user_id" UUID NOT NULL REFERENCES "users"("id") ON DELETE CASCADE,
    "conversation_id" UUID NOT NULL REFERENCES "conversation"("id") ON DELETE CASCADE,
    "role" participant_role DEFAULT 'member',
    "updated_at" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY ("user_id", "conversation_id") -- 复合主键
);

-- FRIENDSHIP (好友关系表)
CREATE TABLE "friendship" (
    "user_id_1" UUID NOT NULL REFERENCES "users"("id") ON DELETE CASCADE,
    "user_id_2" UUID NOT NULL REFERENCES "users"("id") ON DELETE CASCADE,
    "status" friend_status NOT NULL DEFAULT 'pending',
    "requested_by" UUID NOT NULL REFERENCES "users"("id") ON DELETE CASCADE,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY ("user_id_1", "user_id_2"),
    -- 确保 user_id_1 < user_id_2，避免 (A,B) 和 (B,A) 同时存在
    CONSTRAINT "check_user_order" CHECK ("user_id_1" < "user_id_2")
);

-- 4. 创建索引以优化查询性能

CREATE INDEX "idx_message_conversation_id" ON "message" ("conversation_id");
CREATE INDEX "idx_message_created_at" ON "message" ("created_at" DESC);
CREATE INDEX "idx_conversation_participant_user_id" ON "conversation_participant" ("user_id");


-- 5. 插入示例数据 (Sample Data)

DO $$
DECLARE
    -- 定义用户 UUID 变量以便复用
    user_alice_id UUID := 'a1a1a1a1-a1a1-a1a1-a1a1-a1a1a1a1a1a1';
    user_bob_id UUID := 'b2b2b2b2-b2b2-b2b2-b2b2-b2b2b2b2b2b2';
    user_charlie_id UUID := 'c3c3c3c3-c3c3-c3c3-c3c3-c3c3c3c3c3c3';
    user_ai_id UUID := 'a111a111-a111-a111-a111-a111a111a111'; -- AI 助手的固定 ID

    -- 定义会话 UUID 变量
    group_rust_fans_id UUID := 'd4d4d4d4-d4d4-d4d4-d4d4-d4d4d4d4d4d4';
    chat_alice_ai_id UUID := 'e5e5e5e5-e5e5-e5e5-e5e5-e5e5e5e5e5e5';
BEGIN
    -- 插入用户
    INSERT INTO "users" ("id", "username", "password") VALUES
    -- pwd alice123
    (user_alice_id, 'alice', '$2b$12$6xUS4xM0n8.yu3I1hSMnHOzxgklkPMSWZRGEeCKFT3oTvD/erml5q'),
    -- pwd bob123
    (user_bob_id, 'bob', '$2b$12$qSuUi.FqUhuLT2z97xpUwO9iDJ7wQf1cUS3CHN50qK/9oo.9maI3C'),
    -- pwd charlie123
    (user_charlie_id, 'charlie', '$2b$12$9hq1g.VOtNI3t37F4WtdFOT9G0NZlkDcT7/0KOwCfwPh2QDHz8Q8y'),
    (user_ai_id, 'ai_assistant', 'not_applicable'); -- AI助手不需要密码

    -- 插入一个群聊会话: "Rust Fans Club"
    INSERT INTO "conversation" ("id", "type", "name", "creator_id") VALUES
    (group_rust_fans_id, 'group', 'Rust Fans Club', user_alice_id);

    -- 将 Alice, Bob, Charlie 加入该群聊
    -- Alice 是创建者，设为 admin
    INSERT INTO "conversation_participant" ("user_id", "conversation_id", "role") VALUES
    (user_alice_id, group_rust_fans_id, 'admin'),
    (user_bob_id, group_rust_fans_id, 'member'),
    (user_charlie_id, group_rust_fans_id, 'member');

    -- 插入一条系统消息和几条聊天消息
    INSERT INTO "message" ("conversation_id", "sender_id", "type", "content") VALUES
    (group_rust_fans_id, NULL, 'system', '{"text": "Alice created the group."}'), -- 系统消息 sender_id 为 NULL
    (group_rust_fans_id, user_alice_id, 'text', '{"text": "Hey everyone! Welcome to the Rust Fans Club!🦀"}'),
    (group_rust_fans_id, user_bob_id, 'text', '{"text": "Hi Alice! Glad to be here."}');

    -- 插入一个 Alice 与 AI 的会话
    INSERT INTO "conversation" ("id", "type", "name", "creator_id") VALUES
    (chat_alice_ai_id, 'ai_chat', 'AI Assistant Chat', user_alice_id);

    -- 将 Alice 加入该会话
    INSERT INTO "conversation_participant" ("user_id", "conversation_id") VALUES
    (user_alice_id, chat_alice_ai_id);

    -- 插入 Alice 与 AI 的一条对话
    INSERT INTO "message" ("conversation_id", "sender_id", "type", "content") VALUES
    (chat_alice_ai_id, user_alice_id, 'text', '{"text": "Hi AI, can you tell me a joke?"}'),
    (chat_alice_ai_id, user_ai_id, 'text', '{"text": "Why did the Rust programmer break up with the C++ programmer? Because he had too many trust issues!"}');

END $$;
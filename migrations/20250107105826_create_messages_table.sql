-- Создание таблицы topics
CREATE TABLE topics (
                        id SERIAL PRIMARY KEY,                    -- Уникальный идентификатор
                        name TEXT NOT NULL,                       -- Название темы (не может быть NULL)
                        username VARCHAR(255) NOT NULL,          -- Имя пользователя (максимум 255 символов, не может быть NULL)
                        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP -- Время создания (по умолчанию текущее время)
);

-- Создание таблицы messages
CREATE TABLE messages (
                          id SERIAL PRIMARY KEY,                    -- Уникальный идентификатор
                          topic_id INT NOT NULL REFERENCES topics(id) ON DELETE CASCADE, -- Ссылка на тему, удаляется вместе с темой
                          content TEXT NOT NULL,                    -- Содержимое сообщения (не может быть NULL)
                          username VARCHAR(255) NOT NULL,          -- Имя пользователя (максимум 255 символов, не может быть NULL)
                          created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP -- Время создания (по умолчанию текущее время)
);

-- Дополнительно: Создание индекса на `topic_id` в таблице messages для ускорения запросов
CREATE INDEX idx_messages_topic_id ON messages(topic_id);

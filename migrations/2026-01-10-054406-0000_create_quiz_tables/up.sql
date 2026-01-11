-- Your SQL goes here
CREATE TABLE `m_question` (
  `id` bigint(20) NULL AUTO_INCREMENT,
  `question` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_m_quiz` (`id`, `question`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

CREATE TABLE `m_multiple_choice` (
  `id` bigint(20) NULL AUTO_INCREMENT,
  `multiple_choice` varchar(255) DEFAULT NULL,
  `m_question_id` bigint(20) NULL,
  PRIMARY KEY (`id`),
  KEY `idx_m_multiple_choice` (`id`, `multiple_choice`, `m_question_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

CREATE TABLE `m_answer` (
  `id` bigint(20) NULL AUTO_INCREMENT,
  `answer` varchar(255) DEFAULT NULL,
  `multiple_choice_id` bigint(20) NULL,
  `m_question_id` bigint(20) NULL,
  PRIMARY KEY (`id`),
  KEY `idx_m_answer` (`id`, `answer`, `multiple_choice_id`, `m_question_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

CREATE TABLE `m_user` (
  `id` bigint(20) NULL AUTO_INCREMENT,
  `username` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_m_answer` (`id`, `username`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;

CREATE TABLE `m_user_multiple_choice` (
  `id` bigint(20) NULL AUTO_INCREMENT,
  `m_user_id` bigint(20) NULL,
  `m_question_id` bigint(20) NULL,
  `m_multiple_choice_id` bigint(20) NULL,
  `created_on` bigint NULL,
  PRIMARY KEY (`id`),
  KEY `idx_m_user_answer` (`id`, `m_user_id`, `m_question_id`, `m_multiple_choice_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;


pub fn parse_text(input: &str) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut current_sentence = String::new();
    let mut chars = input.chars().peekable();
    let mut dollar_quote = None;
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut in_comment = false;

    while let Some(ch) = chars.next() {
        current_sentence.push(ch);

        if dollar_quote.is_none()
            && ch == '$'
            && !in_single_quote
            && !in_double_quote
            && !in_comment
        {
            let mut tag = String::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch == '$' {
                    chars.next();
                    current_sentence.push(next_ch);
                    break;
                } else {
                    tag.push(next_ch);
                    current_sentence.push(chars.next().unwrap());
                }
            }
            dollar_quote = Some(tag);
        } else if let Some(ref tag) = dollar_quote {
            if ch == '$' {
                let mut end_tag = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '$' {
                        chars.next();
                        current_sentence.push(next_ch);
                        if end_tag == *tag {
                            dollar_quote = None;
                        }
                        break;
                    } else {
                        end_tag.push(next_ch);
                        current_sentence.push(chars.next().unwrap());
                    }
                }
            }
        } else if ch == '\'' && dollar_quote.is_none() && !in_double_quote && !in_comment {
            in_single_quote = !in_single_quote;
        } else if ch == '"' && dollar_quote.is_none() && !in_single_quote && !in_comment {
            in_double_quote = !in_double_quote;
        } else if ch == '/'
            && chars.peek() == Some(&'*')
            && dollar_quote.is_none()
            && !in_single_quote
            && !in_double_quote
        {
            in_comment = true;
            current_sentence.push(chars.next().unwrap());
        } else if ch == '*' && chars.peek() == Some(&'/') && in_comment {
            in_comment = false;
            current_sentence.push(chars.next().unwrap());
        } else if ch == ';'
            && dollar_quote.is_none()
            && !in_single_quote
            && !in_double_quote
            && !in_comment
        {
            sentences.push(current_sentence.trim().to_string());
            current_sentence.clear();
        }
    }

    if !current_sentence.is_empty() {
        sentences.push(current_sentence.trim().to_string());
    }

    sentences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_text() {
        let input = r#"
        CREATE TABLE "User" (
            id BIGSERIAL PRIMARY KEY,
            username VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            email_confirmed_at VARCHAR(255),
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE OR REPLACE FUNCTION update_user_updated_at_column()
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = NOW();
            RETURN NEW;
        END;
        $$ LANGUAGE plpgsql;
        
        CREATE TRIGGER update_user_updated_at BEFORE UPDATE ON "User"
        FOR EACH ROW EXECUTE FUNCTION update_user_updated_at_column();"#;

        let sentences = parse_text(input);

        let exepected = vec![
            r#"CREATE TABLE "User" (
            id BIGSERIAL PRIMARY KEY,
            username VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            email_confirmed_at VARCHAR(255),
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );"#,
            r#"CREATE OR REPLACE FUNCTION update_user_updated_at_column()
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = NOW();
            RETURN NEW;
        END;
        $$ LANGUAGE plpgsql;"#,
            r#"CREATE TRIGGER update_user_updated_at BEFORE UPDATE ON "User"
        FOR EACH ROW EXECUTE FUNCTION update_user_updated_at_column();"#,
        ];

        assert_eq!(
            sentences.len(),
            exepected.len(),
            "The number of sentences does not match the expected number."
        );

        for (i, sentence) in sentences.iter().enumerate() {
            assert_eq!(sentence, &exepected[i]);
        }
    }
}

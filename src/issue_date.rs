use time::Date;
use time::macros::format_description;
use time::format_description::FormatItem;

pub const issue_date_format: &[FormatItem<'_>] = format_description!("[year]-[month]-[day]");

/// takes a "2024-02-12-some-slug" and returns the date portion
pub fn parse_issue_date_from_slug(input: &str) -> Option<time::Date> {
    input.get(0..10).and_then(parse_issue_date)
}

/// takes a "2024-02-12" and returns the date portion
pub fn parse_issue_date(input: &str) -> Option<time::Date> {
    Date::parse(input, &issue_date_format).ok()
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use time::macros::date;

    #[test]
    fn can_parse_date_from_slug() {
        let slug = "2024-02-12-some-slug";
        assert_eq!(Some(date!(2024-02-12)), parse_issue_date_from_slug(slug));
    }

    #[test]
    fn invalid_slug_does_not_parse() {
        let slug = "2024-some-slug";
        assert_eq!(None, parse_issue_date_from_slug(slug));
    }

    #[test]
    fn can_parse_date() {
        let slug = "2024-02-12";
        assert_eq!(Some(date!(2024-02-12)), parse_issue_date(slug));
    }

    #[test]
    fn invalid_date_does_not_parse() {
        let slug = "2024";
        assert_eq!(None, parse_issue_date(slug));
    }
}
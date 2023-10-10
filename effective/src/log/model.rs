use chrono::prelude::*;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord)]
pub struct ServerLogEntry {
    client_ip: String,
    timestamp: DateTime<FixedOffset>,
    request: String,
    status_code: u8,
    size: u32,
}

impl PartialOrd for ServerLogEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

impl TryFrom<&str> for ServerLogEntry {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (client_ip, rest) = value
            .split_once(" ")
            .ok_or(format!("fail to parse ip address from {}", value))?;

        let client_ip = client_ip.to_owned();

        let re =
            Regex::new(r"\[(.*?)\]").map_err(|_| "failed to create timestamp regex".to_owned())?;

        let caps = re
            .captures(rest)
            .ok_or(format!("failed to parse timestamp from {}", rest))?;

        let date_str = caps
            .get(1)
            .ok_or("failed to parse timestamp from caps".to_owned())?
            .as_str();

        let timestamp = DateTime::parse_from_rfc2822(date_str)
            .map_err(|_| "failed to parse timestamp into rfc2822 date".to_owned())?;

        let rest = re.replace(rest, "");

        let re =
            Regex::new("\"(.*?)\"").map_err(|_| "failed to create request regex".to_owned())?;

        let caps = re
            .captures(&rest)
            .ok_or(format!("failed to parse request from {}", rest))?;

        let request = caps
            .get(1)
            .ok_or("failed to get request string from captures")?
            .as_str()
            .to_owned();

        let rest = re.replace(&rest, "");
        let rest = rest.trim();

        let (status, size) = rest
            .split_once(" ")
            .ok_or(format!("failed to parse status and size from {}", rest))?;

        let status_code = status
            .parse()
            .map_err(|_| format!("failed to parse {} into u8", status))?;

        let size = size
            .parse()
            .map_err(|_| format!("failed to parse {} into u32", size))?;

        Ok(Self {
            client_ip,
            timestamp,
            request,
            status_code,
            size,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use super::ServerLogEntry;

    #[test]
    fn parse_log_text_should_works() {
        let log_line =
            "127.0.0.1 [Wed, 18 Feb 2015 23:16:09 GMT] \"GET /apache_pb.gif HTTP/1.0\" 200 2326";

        let expected = ServerLogEntry {
            client_ip: "127.0.0.1".to_owned(),
            timestamp: DateTime::parse_from_rfc2822("Wed, 18 Feb 2015 23:16:09 GMT").unwrap(),
            request: "GET /apache_pb.gif HTTP/1.0".to_string(),
            status_code: 200,
            size: 2326,
        };

        let result: Result<ServerLogEntry, _> = log_line.try_into();

        assert_eq!(result.unwrap(), expected);
    }
}

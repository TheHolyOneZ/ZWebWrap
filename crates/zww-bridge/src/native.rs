use crate::protocol::{Request, Response};
use anyhow::{Context, Result};
use std::io::{Read, Write};

pub const MAX_MESSAGE: u32 = 1024 * 1024;

pub fn read_request<R: Read>(input: &mut R) -> Result<Option<Request>> {
    let mut len_bytes = [0u8; 4];
    match input.read_exact(&mut len_bytes) {
        Ok(()) => {}

        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e).context("reading the message length"),
    }

    let len = u32::from_ne_bytes(len_bytes);
    anyhow::ensure!(len <= MAX_MESSAGE, "message of {len} bytes is over the limit");
    if len == 0 {
        return Ok(None);
    }

    let mut buf = vec![0u8; len as usize];
    input.read_exact(&mut buf).context("reading the message body")?;

    let request = serde_json::from_slice(&buf).context("message was not a known request")?;
    Ok(Some(request))
}

pub fn write_response<W: Write>(output: &mut W, response: &Response) -> Result<()> {
    let body = serde_json::to_vec(response).context("serialising the response")?;
    anyhow::ensure!(
        body.len() as u32 <= MAX_MESSAGE,
        "response of {} bytes is over the limit",
        body.len()
    );

    output.write_all(&(body.len() as u32).to_ne_bytes())?;
    output.write_all(&body)?;

    output.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::Response;

    fn framed(request: &Request) -> Vec<u8> {
        let body = serde_json::to_vec(request).unwrap();
        let mut out = (body.len() as u32).to_ne_bytes().to_vec();
        out.extend(body);
        out
    }

    #[test]
    fn reads_a_framed_request() {
        let request = Request::Ping;
        let mut input = std::io::Cursor::new(framed(&request));
        assert_eq!(read_request(&mut input).unwrap(), Some(request));
    }

    #[test]
    fn reads_several_messages_from_one_stream() {
        let mut bytes = framed(&Request::Ping);
        bytes.extend(framed(&Request::ListApps));
        let mut input = std::io::Cursor::new(bytes);

        assert_eq!(read_request(&mut input).unwrap(), Some(Request::Ping));
        assert_eq!(read_request(&mut input).unwrap(), Some(Request::ListApps));

        assert_eq!(read_request(&mut input).unwrap(), None);
    }

    #[test]
    fn a_closed_pipe_is_not_an_error() {
        let mut empty = std::io::Cursor::new(Vec::new());
        assert_eq!(read_request(&mut empty).unwrap(), None);
    }

    #[test]
    fn a_truncated_body_is_an_error_not_a_silent_stop() {
        let mut bytes = 100u32.to_ne_bytes().to_vec();
        bytes.extend(b"{}\n\n");
        let mut input = std::io::Cursor::new(bytes);
        assert!(read_request(&mut input).is_err());
    }

    #[test]
    fn an_oversized_length_is_refused_before_allocating() {
        let mut input = std::io::Cursor::new((MAX_MESSAGE + 1).to_ne_bytes().to_vec());
        let err = read_request(&mut input).unwrap_err().to_string();
        assert!(err.contains("over the limit"), "got {err}");
    }

    #[test]
    fn responses_are_written_with_a_matching_prefix() {
        let mut out = Vec::new();
        write_response(&mut out, &Response::Ok).unwrap();

        let len = u32::from_ne_bytes(out[..4].try_into().unwrap()) as usize;
        assert_eq!(len, out.len() - 4);
        assert_eq!(serde_json::from_slice::<Response>(&out[4..]).unwrap(), Response::Ok);
    }

    #[test]
    fn a_write_and_read_pair_round_trips() {
        let mut buf = Vec::new();
        write_response(&mut buf, &Response::error("boom")).unwrap();
        let len = u32::from_ne_bytes(buf[..4].try_into().unwrap()) as usize;
        let parsed: Response = serde_json::from_slice(&buf[4..4 + len]).unwrap();
        assert_eq!(parsed, Response::error("boom"));
    }
}

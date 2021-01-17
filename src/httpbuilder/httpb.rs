pub struct HttpBuilder {
    response_code: u16,
    content: String,
}

impl HttpBuilder {
    /// Sets the http response code
    ///
    /// The response code
    ///
    /// Returns Err(()) on error
    pub fn set_code(&mut self, code: u16) -> Result<(), ()> {
        if code < 600 {
            self.response_code = code;
            return Ok(());
        }
        Err(())
    }
    /// Sets the content string to the argument given
    ///
    /// The content
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    /// Builds the http response
    pub fn build(&self) -> String {
        format!(
            "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}",
            self.response_code,
            self.content.len(),
            self.content
        )
    }
}

impl Default for HttpBuilder {
    fn default() -> HttpBuilder {
        HttpBuilder {
            response_code: 0,
            content: "".to_string(),
        }
    }
}

pub struct StrSplit<'haystack, 'delimiter>{
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
 }


impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
     }

}


impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {

    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {

        if let Some(ref mut remainder) = self.remainder {

            if let Some(next_delim) = remainder.find(self.delimiter) {

                let until_delimiter = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delimiter)

            } else {
                self.remainder.take()
            }

        } else {
            None
        }

    }
}


fn until_char(s: &str, c: char) -> &str {
    let delim = format!("{}", c);
    StrSplit::new(s, &delim).next().expect("StrSplit always gives one result")

}


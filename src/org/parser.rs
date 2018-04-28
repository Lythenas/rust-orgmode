use org::*;

/// Error returned by [`parse_node`]. The variants should be self expanatory.
pub enum OrgNodeParseError {
    ExpectedNewHeadline,
}

/// Parse a node (and recursively all of its sub-nodes) from the given string.
/// 
/// Returns an error if it doesn't find a correctly formatted headline at the start of the
/// given string. Stops when the string ends or if it finds another headline with same level.
pub fn parse_node(text: &str) -> Result<OrgNode, OrgNodeParseError> {
    let mut lines = text.lines();

    let first_line = lines.next();
    let second_line = lines.next();

    let level = count_prefix_chars(text, '*');

    if (level == 0) {
        return Err(OrgNodeParseError::ExpectedNewHeadline);
    }

    Ok(OrgNode::default())
}

fn count_prefix_chars(s: &str, needle: char) -> usize {
    s.chars().take_while(|c| c == &needle).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_prefix_chars() {
        assert_eq!(count_prefix_chars("* abc", '*'), 1);
        assert_eq!(count_prefix_chars("*** abc *", '*'), 3);
        assert_eq!(count_prefix_chars("****** abc ** asd *", '*'), 6);
        assert_eq!(count_prefix_chars("* abc ** a", '*'), 1);
        assert_eq!(count_prefix_chars("abs * abc", '*'), 0);
    }
    
}

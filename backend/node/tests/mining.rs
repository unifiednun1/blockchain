#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_calculate_nun_reward() {
        use crate::ai_integration::calculate_nun_reward;
        assert_eq!(calculate_nun_reward("click", 1.0), 1);
        assert_eq!(calculate_nun_reward("scroll", 1.0), 0);
        assert_eq!(calculate_nun_reward("visit", 1.0), 2);
        assert_eq!(calculate_nun_reward("other", 2.0), 2);
    }
}

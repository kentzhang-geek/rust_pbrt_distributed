#[cfg(test)]
mod tests {
    use rpd::interface::shape;
    use rpd::interface::shape::Shape;

    #[test]
    fn test01() {
        let s = shape::STest{v:12i32};
        s.test();
    }
}

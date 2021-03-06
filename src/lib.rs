pub mod utils {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    // A wrapper for native C API.
    pub fn get_abs(x: i32) -> i32 {
        unsafe { abs(x) }
    }

    #[test]
    fn test_get_abs() {
        assert_eq!(get_abs(0), 0);
        assert_eq!(get_abs(60), 60);
        assert_eq!(get_abs(-60), 60);
    }
}

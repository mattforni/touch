mod create {
    mod when_dir_exists {
        use dir::create;

        #[test]
        fn it_returns_ok() {
            // Arrange / Act / Assert
            assert!(create("/").is_ok());
        }
    }

    mod when_bad_permissions {
        use dir::create;

        #[test]
        fn it_returns_err() {
            // Arrange / Act / Assert
            assert!(create("/restricted").is_err());
        }
    }

    mod when_parent_dne {
        use exists;
        use dir::{create,delete};

        #[test]
        fn it_returns_ok_and_creates_parent() {
            // Arrange
            let parent = "/tmp/parent";
            assert!(!exists(parent));
            let dir = &format!("{}/child", parent);
            assert!(!exists(dir));

            // Act / Assert
            assert!(create(dir).is_ok());

            // Cleanup
            assert!(delete(parent).is_ok());
        }
    }
}

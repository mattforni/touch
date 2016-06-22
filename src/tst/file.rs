mod create {
    mod when_file_exists {
        mod when_not_truncate {
            use std::fs::{self,File};

            use exists;
            use file::create;

            #[test]
            fn it_returns_ok_with_none() {
                // Arrange
                let path = "/tmp/no_truncate.txt";
                let file = File::create(path);
                assert!(file.is_ok());
                assert!(exists(path));

                // Act
                let file = create(path, false);

                // Assert
                assert!(file.is_ok());
                assert!(file.unwrap().is_none());

                // Cleanup
                assert!(fs::remove_file(path).is_ok());
            }
        }

        mod when_truncate {
            use std::fs::{self,File};

            use exists;
            use file::create;

            #[test]
            fn it_returns_ok_with_file() {
                // Arrange
                let path = "/tmp/truncate.txt";
                let file = File::create(path);
                assert!(file.is_ok());
                assert!(exists(path));

                // Act
                let file = create(path, true);

                // Assert
                assert!(file.is_ok());
                assert!(file.unwrap().is_some());

                // Cleanup
                assert!(fs::remove_file(path).is_ok());
            }
        }
    }

    mod when_file_dne {
        mod when_bad_permissions {
            use file::create;

            #[test]
            fn it_returns_err() {
                // Arrange / Act / Assert
                assert!(create("/tst.txt", true).is_err());
            }
        }

        mod when_dir_dne {
            use dir::delete;
            use exists;
            use file::create;

            #[test]
            fn it_returns_ok_and_creates_dir_and_file() {
                // Arrange
                let dir = "/tmp/dne";
                assert!(!exists(dir));

                // Act / Assert
                let file = "/tmp/dne/.test";
                assert!(create(file, true).is_ok());
                assert!(exists(dir));
                assert!(exists(file));

                // Cleanup
                assert!(delete(dir).is_ok());
            }
        }

        use std::fs;

        use exists;
        use file::create;

        #[test]
        fn it_returns_ok_with_file() {
            // Arrange
            let path = "/tmp/new_file.txt";

            // Act
            let file = create(path, true);

            // Assert
            assert!(file.is_ok());
            assert!(file.unwrap().is_some());
            assert!(exists(path));

            // Cleanup
            assert!(fs::remove_file(path).is_ok());
        }
    }
}

mod delete {
    mod when_file_dne {
        use exists;
        use file::delete;

        #[test]
        fn it_returns_ok() {
            // Arrange
            let path = "/dne.txt";
            assert!(!exists(path));

            // Act / Assert
            assert!(delete(path).is_ok());
        }
    }

    use std::fs::File;

    use exists;
    use file::delete;

    #[test]
    fn it_returns_ok_and_deletes_file() {
        // Arrange
        let path = "/tmp/.delete";
        let file = File::create(path);
        assert!(file.is_ok(), file.err().unwrap().to_string());
        assert!(exists(path));

        // Act / Assert
        assert!(delete(path).is_ok());
        assert!(!exists(path));
    }
}

mod read {
    mod when_file_dne {
        use exists;
        use file::read;

        #[test]
        fn it_returns_err() {
            // Arrange
            let path = "/dne.txt";
            assert!(!exists(path));

            // Act / Assert
            assert!(read(path).is_err());
        }
    }

    use std::fs::File;
    use std::io::Write;

    use file::{delete,read};

    #[test]
    fn if_returns_ok_with_content() {
        // Arrange
        let path = "/tmp/.read";
        let file = File::create(path);
        assert!(file.is_ok(), file.err().unwrap().to_string());
        let content = "content";
        assert!(file.unwrap().write_all(content.as_bytes()).is_ok());

        // Act
        let read = read(path);

        // Assert
        assert!(read.is_ok());
        assert_eq!(content, read.unwrap());

        // Cleanup
        assert!(delete(path).is_ok())
    }
}

mod write {
    mod when_file_dne {
        use exists;
        use file::{delete,read,write};

        #[test]
        fn it_returns_ok_and_create_file_with_content() {
            // Arrange
            let path = "/tmp/.dne";
            assert!(!exists(path));

            // Act / Assert
            let content = "content";
            assert!(write(path, content, false).is_ok());
            assert!(exists(path));
            let read = read(path);
            assert!(read.is_ok());
            assert_eq!(content, read.unwrap());

            // Cleanup
            assert!(delete(path).is_ok());
        }
    }

    mod when_content_empty {
        use exists;
        use file::write;

        #[test]
        fn it_returns_ok_and_does_not_create_file() {
            // Arrange
            let path = "/tmp/.write";
            assert!(!exists(path));

            // Act / Assert
            assert!(write(path, "", false).is_ok());
            assert!(!exists(path));
        }
    }

    mod when_truncate_true {
        use exists;
        use file::{delete,read,write};

        #[test]
        fn it_returns_ok_and_overwrites_content() {
            // Arrange
            let path = "/tmp/.write";
            let content = "content";
            assert!(write(path, content, false).is_ok());
            assert!(exists(path));
            let mut output = read(path);
            assert!(output.is_ok());
            assert_eq!(content, output.unwrap());

            // Act / Assert
            let overwrite = "overwrite";
            assert!(write(path, overwrite, true).is_ok());
            output = read(path);
            assert!(output.is_ok());
            assert_eq!(overwrite, output.unwrap());

            // Cleanup
            assert!(delete(path).is_ok());
        }
    }
}

extern crate rcudnn as cudnn;

#[cfg(test)]
mod tensor_descriptor_spec {

    use cudnn::TensorDescriptor;
    use cudnn::utils::DataType;

    #[test]
    fn it_initializes_a_tensor_descriptor() {
        match TensorDescriptor::new(&[2, 2, 2], &[4, 2, 1], DataType::Float) {
            Ok(_) => assert!(true),
            Err(err) => {
                println!("{:?}", err);
                assert!(false);
            }
        }
    }
}

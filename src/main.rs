fn main() {
    let tensor_1 = tensor::Tensor::new(vec![2, 3], vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
    let mut tensor = match tensor_1 {
        Ok(value) => {
            println!("Created a Tensor");
            value
        }
        Err(error) => {
            panic!("An Error Occured, {}", error);
        }
    };

    println!("Stride: {:?}", tensor.stride);
    println!("Shape: {:?}", tensor.shape);

    tensor.reshape(vec![3, 2]).expect("Error reshaping Tensor!");

    println!("Stride: {:?}", tensor.stride);
    println!("Shape2: {:?}", tensor.shape);

    let tensor_view = tensor::TensorView::new(&tensor);

    println!("Stride: {:?}", tensor_view.stride);
    println!("Shape2: {:?}", tensor_view.shape);
}

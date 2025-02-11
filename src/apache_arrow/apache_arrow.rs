use std::sync::Arc;
use arrow::array::{Int32Array, StringArray};
use arrow::datatypes::{Field, Schema, DataType};
use arrow::record_batch::RecordBatch;

pub fn apache_arrow_demo()-> Result<(), Box<dyn std::error::Error>> {
    // 创建一个 Schema
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("name", DataType::Utf8, false),
    ]);

    // 创建两个数组
    let id_array = Int32Array::from(vec![1, 2, 3]);
    let name_array = StringArray::from(vec!["Alice", "Bob", "Charlie"]);

    // 创建一个 RecordBatch
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![Arc::new(id_array), Arc::new(name_array)],
    )?;

    // 打印 RecordBatch
    println!("{:?}", batch);

    Ok(())
}
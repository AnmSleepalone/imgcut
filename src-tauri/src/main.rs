// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::GenericImageView;
use image::imageops::crop;
use walkdir::WalkDir;
use std::fs;
use std::path::Path;
use tokio::sync::mpsc;
use anyhow::Result;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
enum ProcessMessage {
    Progress(String),
    Error(String),
    Completed,
}

#[derive(Error, Debug)]
enum ProcessError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Send error: {0}")]
    Send(#[from] tokio::sync::mpsc::error::SendError<ProcessMessage>),
    #[error("{0}")]
    Custom(String),
}

#[tauri::command]
async fn greet(
    window: tauri::Window,
    path: String,
    output_dir: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32
) -> Result<String, String> {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        if let Err(e) = process_images(path, output_dir, x, y, width, height, tx).await {
            println!("Error: {}", e);
        }
    });

    // 使用事件发送进度更新
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                ProcessMessage::Progress(message) => {
                    window.emit("greet-progress", message).unwrap();
                }
                ProcessMessage::Error(error) => {
                    window.emit("greet-error", error).unwrap();
                }
                ProcessMessage::Completed => {
                    window.emit("greet-completed", ()).unwrap();
                    break;
                }
            }
        }
    });

    Ok("全部完成".to_string())
}

async fn process_images(
    path: String,
    output_dir: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    tx: mpsc::Sender<ProcessMessage>
) -> Result<(), ProcessError> {
    fs::create_dir_all(&output_dir)?;
    tx.send(ProcessMessage::Progress(format!("创建输出目录: {}", output_dir))).await?;

    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            match crop_and_save_image(path, &output_dir, x, y, width, height) {
                Ok(_) => {
                    let message = format!("处理完成一张: {:?}", path);
                    println!("{}", message);
                    tx.send(ProcessMessage::Progress(message)).await?;
                }
                Err(e) => {
                    let error = format!("处理出错 {:?}: {}", path, e);
                    println!("{}", error);
                    tx.send(ProcessMessage::Error(error)).await?;
                }
            }
        }
    }

    tx.send(ProcessMessage::Completed).await?;
    Ok(())
}

fn crop_and_save_image<P: AsRef<Path>>(path: P, output_dir: &str, crop_x: u32, crop_y: u32, width: u32, height: u32) -> Result<(), ProcessError> {
    let mut img = image::open(&path)?;
    let (img_width, img_height) = img.dimensions();

    // 确保裁剪尺寸不超过图片尺寸
    if width > img_width || height > img_height {
        return Err(ProcessError::Custom("裁剪尺寸不能超过图片尺寸!".into()));
    }

    // 从左上角为原点裁剪图片
    let cropped_img = crop(&mut img, crop_x, crop_y, width, height).to_image();
    let output_path = Path::new(output_dir).join(path.as_ref().file_name().unwrap());
    cropped_img.save(output_path)?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("Tauri 运行失败!");
}
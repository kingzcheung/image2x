// Copyright 2022 kingzcheung
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{fs, io::Write};

use optimize::{OptimizeImage, Jpeg, error::OptimizeError};
use serde::Serialize;

pub mod convert;
pub mod optimize;
pub mod yolo;

#[derive(Debug, Serialize)]
pub struct Meta {
    file_size: u64,
    file: String,
}

/// 获取文件meta信息
pub async fn file_metadata(files: Vec<String>) -> Result<Vec<Meta>, anyhow::Error> {
    let mut meta = vec![];
    for file in files {
        let m = fs::metadata(file.as_str())?;
        meta.push(Meta {
            file,
            file_size: m.len(),
        })
    }
    Ok(meta)
}

/// JPEG / PNG 压缩
pub async fn image_optimize(filename:String,compression_level:u8) ->Result<usize,OptimizeError>{
    let oimg = OptimizeImage::new(filename.as_str(), compression_level)?;
    let img = oimg.optimize(Jpeg)?;
    let size = img.len();
    // save
    let mut f = std::fs::File::create(filename).map_err(OptimizeError::IOError)?;
    f.write(img.as_slice()).map_err(OptimizeError::IOError)?;
    Ok(size)
}
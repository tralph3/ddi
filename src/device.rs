//A safer dd
//Copyright (C) 2021  Tomás Ralph
//
//This program is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation, either version 3 of the License, or
//(at your option) any later version.
//
//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License
//along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
////////////////////////////////////
//                                //
//       Created by tralph3       //
//   https://github.com/tralph3   //
//                                //
////////////////////////////////////

use std::process::{self, ExitStatus};

pub struct DeviceInfo {
    pub name: String,
    pub model: String,
    pub size: String,
    pub partitions: Vec<PartitionInfo>
}

pub struct PartitionInfo {
    pub name: String,
    pub fs: String,
    pub mount: String,
    pub label: String,
    pub size: String
}

impl DeviceInfo {
    pub fn new(device: &str) -> Result<DeviceInfo, ExitStatus> {

        fn get_device_data(device: &str) -> Result<serde_json::Value, ExitStatus> {

            let data = process::Command::new("lsblk")
                .args(&["-JO", device])
                .output()
                .unwrap();
            let values = std::str::from_utf8(&values).unwrap();

            if data.status.code() == Some(32) {
                return Err(data.status);
            }

            let parsed_data: serde_json::Value =
                serde_json::from_str(values).unwrap();

            Ok(parsed_data)
        }

        fn parse_device(device_data: serde_json::Value) -> DeviceInfo {

            let block_device = &device_data["blockdevices"][0];

            let name: String = block_device["name"].to_string();
            let model: String = block_device["model"].to_string();
            let size: String = block_device["size"].to_string();
            let partitions: &serde_json::Value = &block_device["children"];

            let mut part_vector: Vec<PartitionInfo> = Vec::new();

            if !partitions.is_null() {

                for partition in partitions.as_array().unwrap() {

                    let part_name  = partition["name"].to_string();
                    let part_fs    = partition["fstype"].to_string();
                    let part_mount = partition["mountpoint"].to_string();
                    let part_label = partition["label"].to_string();
                    let part_size  = partition["size"].to_string();

                    part_vector.push(
                        PartitionInfo {
                            name: part_name,
                            fs: part_fs,
                            mount: part_mount,
                            label: part_label,
                            size: part_size
                        }
                    )
                }
            }

            DeviceInfo {
                name,
                model,
                size,
                partitions: part_vector
            }

        }

        let device_data = get_device_data(device);

        if let Err(e) = device_data {
            return Err(e)
        };

        let device_info = parse_device(device_data.unwrap());

        Ok(device_info)
    }
}

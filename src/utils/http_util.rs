use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::{anyhow, Result};
use md5::{Digest, Md5};
// use reqwest::multipart::{Form, Part};
use url::form_urlencoded;
// use reqwest::blocking::Client;

// 不再需要导入tokio运行时，因为我们使用reqwest的阻塞客户端

use crate::config;

/// 执行基本的HTTP请求
///
/// # 参数
/// * `t` - 时间戳
/// * `param` - 请求参数
/// * `post_url` - 请求URL
pub fn do_request(t: &str, param: &str, post_url: &str) -> Result<String> {
    // 计算签名
    let sign_str = format!("{}{}{}{}", param, t, config::account::KEY, config::account::SECRET);
    let sign = calculate_sign(&sign_str);

    // 构造form表单数据
    let mut form_data = HashMap::new();
    form_data.insert("key", config::account::KEY);
    form_data.insert("t", t);
    form_data.insert("sign", &sign);
    form_data.insert("param", param);

    execute(post_url, form_data)
}

/// 执行带方法的HTTP请求
///
/// # 参数
/// * `method` - 方法名
/// * `t` - 时间戳
/// * `param` - 请求参数
/// * `post_url` - 请求URL
pub fn do_method_request(method: &str, t: &str, param: &str, post_url: &str) -> Result<String> {
    // 计算签名
    let sign_str = format!("{}{}{}{}", param, t, config::account::KEY, config::account::SECRET);
    let sign = calculate_sign(&sign_str);

    // 构造form表单数据
    let mut form_data = HashMap::new();
    form_data.insert("key", config::account::KEY);
    form_data.insert("method", method);
    form_data.insert("t", t);
    form_data.insert("sign", &sign);
    form_data.insert("param", param);

    execute(post_url, form_data)
}

/// 使用customer鉴权的请求
///
/// # 参数
/// * `param` - 请求参数
/// * `post_url` - 请求URL
pub fn customer_request(param: &str, post_url: &str) -> Result<String> {
    // 计算签名
    let sign_str = format!("{}{}{}", param, config::account::KEY, config::account::CUSTOMER);
    let sign = calculate_sign(&sign_str);

    // 构造form表单数据
    let mut form_data = HashMap::new();
    form_data.insert("customer", config::account::CUSTOMER);
    form_data.insert("sign", &sign);
    form_data.insert("param", param);

    execute(post_url, form_data)
}

/// 根据map传入form数据的请求
///
/// # 参数
/// * `m` - 包含表单数据的HashMap
/// * `post_url` - 请求URL
pub fn do_map_request(m: HashMap<&str, &str>, post_url: &str) -> Result<String> {
    execute(post_url, m)
}

/// 带文件上传的请求
///
/// # 参数
/// * `m` - 包含表单数据的HashMap
/// * `file_path` - 文件路径
/// * `post_url` - 请求URL
pub fn do_file_request(m: HashMap<&str, &str>, file_path: &str, post_url: &str) -> Result<String> {
    let path = Path::new(file_path);
    let file_name = path.file_name()
        .ok_or_else(|| anyhow!("无效的文件路径"))?;
    let file_name = file_name.to_string_lossy();

    // 打开文件
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // 创建multipart表单
    let mut form = reqwest::blocking::multipart::Form::new();

    // 接口请求参数
    let mut request_param = String::new();

    // 添加map中的数据到表单
    for (key, value) in &m {
        form = form.text(key.to_string(), value.to_string());
        request_param.push_str(&format!(" {}：{}", key, value));
    }

    // 添加文件到表单
    let part = reqwest::blocking::multipart::Part::bytes(buffer)
        .file_name(file_name.to_string());
    form = form.part("file", part);

    // 创建HTTP客户端（使用阻塞客户端）
    let client = reqwest::blocking::Client::new();

    // 发送HTTP请求
    println!("请求URL: {}", post_url);
    println!("请求参数: {}", request_param);

    let resp = client.post(post_url)
        .multipart(form)
        .send()?;

    let status = resp.status();
    let body = resp.text()?;

    if !status.is_success() {
        println!("请求失败: {}", status);
        return Err(anyhow!("请求失败: {}", status));
    }

    // 打印响应内容
    println!("响应内容: {}", body);
    Ok(body)
}

/// 计算MD5签名并转为大写
fn calculate_sign(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let hex_string = hex::encode(result);
    hex_string.to_uppercase()
}

/// 执行HTTP请求 (同步版本)
fn execute(post_url: &str, form_data: HashMap<&str, &str>) -> Result<String> {
    // 创建HTTP客户端
    let client = reqwest::blocking::Client::new();

    // 构建表单数据
    let form_body: String = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(form_data.iter())
        .finish();

    // 发送HTTP请求
    println!("请求URL: {}", post_url);
    println!("请求参数: {:?}", form_data);

    let resp = client.post(post_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_body)
        .send()?;

    let status = resp.status();
    let body = resp.text()?;

    if !status.is_success() {
        println!("请求失败: {}", status);
        return Err(anyhow!("请求失败: {}", status));
    }

    // 打印响应内容
    println!("响应内容: {}", body);
    Ok(body)
}

/// 执行HTTP请求 (异步版本)
async fn execute_async(post_url: &str, form_data: HashMap<&str, &str>) -> Result<String> {
    // 创建HTTP客户端
    let client = reqwest::Client::new();

    // 构建表单数据
    let form_body: String = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(form_data.iter())
        .finish();

    // 发送HTTP请求
    println!("请求URL: {}", post_url);
    println!("请求参数: {:?}", form_data);

    let resp = client.post(post_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_body)
        .send()
        .await?;

    let status = resp.status();
    let body = resp.text().await?;

    if !status.is_success() {
        println!("请求失败: {}", status);
        return Err(anyhow!("请求失败: {}", status));
    }

    // 打印响应内容
    println!("响应内容: {}", body);
    Ok(body)
}
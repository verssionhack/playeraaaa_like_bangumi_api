use crypto::symmetriccipher::SymmetricCipherError;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{WriteBuffer, ReadBuffer, RefReadBuffer, RefWriteBuffer, BufferResult::BufferUnderflow};
use crypto::aes::{self, KeySize::KeySize128};

pub fn get_url(url: &str, token_key: &[u8], token_iv: &[u8]) -> Option<String> {
    std::str::from_utf8(&aes128_cbc_decrypt(&base64::decode(url.as_bytes()).unwrap(), token_key, token_iv).ok()?).map(|v| v.to_string()).ok()
}

fn aes128_cbc_encrypt(
    data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(
        KeySize128,
        key, iv,
        PkcsPadding,
    );

    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);
    let mut read_buffer = RefReadBuffer::new(data);
    let mut final_result = Vec::new();

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferUnderflow => break,
            _ => continue,
        }
    }

    Ok(final_result)
}

fn aes128_cbc_decrypt(
    data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
        KeySize128,
        key, iv,
        PkcsPadding,
    );

    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);
    let mut read_buffer = RefReadBuffer::new(data);
    let mut final_result = Vec::new();

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferUnderflow => break,
            _ => continue,
        }
    }

    Ok(final_result)
}


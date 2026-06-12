#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, String, symbol_short,
};

#[contracttype]
#[derive(Clone)]
pub struct Certificate {
    pub student_name: String,
    pub major: String,
    pub issue_date: String,
    pub valid: bool,
}

#[contract]
pub struct CertificateContract;

#[contractimpl]
impl CertificateContract {

    // Tạo chứng chỉ
    pub fn create_certificate(
        env: Env,
        id: String,
        student_name: String,
        major: String,
        issue_date: String,
    ) {
        let cert = Certificate {
            student_name,
            major,
            issue_date,
            valid: true,
        };

        env.storage().persistent().set(&id, &cert);
    }

    // Lấy thông tin chứng chỉ
    pub fn get_certificate(
        env: Env,
        id: String,
    ) -> Option<Certificate> {
        env.storage().persistent().get(&id)
    }

    // Xác minh
    pub fn verify_certificate(
        env: Env,
        id: String,
    ) -> bool {

        match env.storage().persistent().get::<String, Certificate>(&id) {
            Some(cert) => cert.valid,
            None => false,
        }
    }

    // Thu hồi chứng chỉ
    pub fn revoke_certificate(
        env: Env,
        id: String,
    ) {
        if let Some(mut cert) =
            env.storage().persistent().get::<String, Certificate>(&id)
        {
            cert.valid = false;
            env.storage().persistent().set(&id, &cert);
        }
    }

    // Đếm số lượng chứng chỉ đã tạo
    pub fn increment_count(env: Env) {
        let key = symbol_short!("COUNT");

        let count: u32 =
            env.storage()
                .persistent()
                .get(&key)
                .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&key, &(count + 1));
    }

    pub fn get_count(env: Env) -> u32 {
        let key = symbol_short!("COUNT");

        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(0)
    }
}
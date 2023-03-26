mod photon;

pub use photon::*;

use image::ImageOutputFormat;

use crate::pb::Spec;

/// Engine trait, 图片处理引擎
pub trait ImageEngine {
    // 按 specs 顺序进行有序处理
    fn apply(&mut self, specs: &[Spec]);

    // 生成目标图片，消耗原来的值，而不是引用
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

/// SpecTransformer, Spec(T) 的转换功能接口
pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}

use crate::graph::grad::Gradients;
use crate::tensor::backend::ADBackend;
use crate::tensor::backend::Backend;
use crate::tensor::ops::activation::*;
use crate::tensor::ops::*;
use crate::tensor::{Data, Distribution, Shape};
use crate::BoolTensor;
use crate::Element;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct Tensor<B: Backend, const D: usize> {
    pub(crate) value: B::TensorPrimitive<D>,
}

impl<const D: usize, B> Tensor<B, D>
where
    B: Backend,
{
    pub fn new(tensor: B::TensorPrimitive<D>) -> Self {
        Self { value: tensor }
    }

    pub fn reshape<const D2: usize>(&self, shape: Shape<D2>) -> Tensor<B, D2> {
        Tensor::new(self.value.reshape(shape))
    }

    pub fn to_device(&self, device: B::Device) -> Self {
        Self::new(self.value.to_device(device))
    }

    pub fn exp(&self) -> Self {
        Self::new(self.value.exp())
    }

    pub fn log(&self) -> Self {
        Self::new(self.value.log())
    }

    pub fn device(&self) -> B::Device {
        self.value.device()
    }

    pub fn shape(&self) -> &Shape<D> {
        self.value.shape()
    }

    pub fn into_data(self) -> Data<B::Elem, D> {
        self.value.into_data()
    }

    pub fn to_data(&self) -> Data<B::Elem, D> {
        self.value.to_data()
    }

    pub fn zeros_like(&self) -> Self {
        Tensor::new(B::zeros(self.shape().clone(), self.value.device()))
    }

    pub fn one_hot(index: usize, num_classes: usize) -> Self {
        let mut dims = [1; D];
        dims[D - 1] = num_classes;
        let shape = Shape::new(dims);
        let tensor = Tensor::zeros(shape);
        let ranges: Vec<_> = shape.dims.iter().map(|dim| 0..dim.clone()).collect();
        let mut ranges: [std::ops::Range<usize>; D] = ranges.try_into().unwrap();
        ranges[D - 1] = index..index + 1;

        let tensor = tensor.index_assign(ranges, &Tensor::ones(Shape::new([1; D])));

        tensor
    }

    pub fn ones_like(&self) -> Self {
        Tensor::new(B::ones(self.shape().clone(), self.value.device()))
    }

    pub fn random_like(&self, distribution: Distribution<B::Elem>) -> Self {
        Tensor::new(B::random(
            self.shape().clone(),
            distribution,
            self.value.device(),
        ))
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(self.value.add(&other.value))
    }

    pub fn add_scalar(&self, other: &B::Elem) -> Self {
        Self::new(self.value.add_scalar(&other))
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self::new(self.value.sub(&other.value))
    }

    pub fn sub_scalar(&self, other: &B::Elem) -> Self {
        Self::new(self.value.sub_scalar(&other))
    }

    pub fn transpose(&self) -> Self {
        Self::new(self.value.transpose())
    }

    pub fn matmul(&self, other: &Self) -> Self {
        Self::new(self.value.matmul(&other.value))
    }

    pub fn neg(&self) -> Self {
        Self::new(self.value.neg())
    }

    pub fn mul(&self, other: &Self) -> Self {
        Self::new(self.value.mul(&other.value))
    }

    pub fn mul_scalar(&self, other: &B::Elem) -> Self {
        Self::new(self.value.mul_scalar(&other))
    }

    pub fn div(&self, other: &Self) -> Self {
        Self::new(self.value.div(&other.value))
    }

    pub fn div_scalar(&self, other: &B::Elem) -> Self {
        Self::new(self.value.div_scalar(&other))
    }

    pub fn random(shape: Shape<D>, distribution: Distribution<B::Elem>) -> Self {
        let tensor = B::random(shape, distribution, B::Device::default());
        Self::new(tensor)
    }

    pub fn mean(&self) -> Tensor<B, 1> {
        Tensor::new(self.value.mean())
    }

    pub fn sum(&self) -> Tensor<B, 1> {
        Tensor::new(self.value.sum())
    }

    pub fn mean_dim(&self, dim: usize) -> Self {
        Self::new(self.value.mean_dim(dim))
    }

    pub fn sum_dim(&self, dim: usize) -> Self {
        Self::new(self.value.sum_dim(dim))
    }

    pub fn equal(&self, other: &Self) -> BoolTensor<B, D> {
        BoolTensor::new(self.value.equal(&other.value))
    }

    pub fn equal_scalar(&self, other: &B::Elem) -> BoolTensor<B, D> {
        BoolTensor::new(self.value.equal_scalar(other))
    }

    pub fn greater(&self, other: &Self) -> BoolTensor<B, D> {
        BoolTensor::new(self.value.greater(&other.value))
    }

    pub fn greater_equal(&self, other: &Self) -> BoolTensor<B, D> {
        BoolTensor::new(self.value.greater_equal(&other.value))
    }

    pub fn greater_scalar(&self, other: &B::Elem) -> BoolTensor<B, D> {
        BoolTensor::new(self.value.greater_scalar(other))
    }

    pub fn greater_equal_scalar(&self, other: &B::Elem) -> BoolTensor<B, D> {
        BoolTensor::new(self.value.greater_equal_scalar(other))
    }

    pub fn lower(&self, other: &Self) -> BoolTensor<B, D> {
        BoolTensor::new(self.value.lower(&other.value))
    }

    pub fn lower_equal(&self, other: &Self) -> BoolTensor<B, D> {
        BoolTensor::new(self.value.lower_equal(&other.value))
    }

    pub fn lower_scalar(&self, other: &B::Elem) -> BoolTensor<B, D> {
        BoolTensor::new(self.value.lower_scalar(other))
    }

    pub fn lower_equal_scalar(&self, other: &B::Elem) -> BoolTensor<B, D> {
        BoolTensor::new(self.value.lower_equal_scalar(other))
    }

    pub fn zeros(shape: Shape<D>) -> Self {
        let tensor = B::zeros(shape, B::Device::default());
        Self::new(tensor)
    }

    pub fn ones(shape: Shape<D>) -> Self {
        let tensor = B::ones(shape, B::Device::default());
        Self::new(tensor)
    }

    pub fn from_data(data: Data<B::Elem, D>) -> Self {
        let tensor = B::from_data(data, B::Device::default());
        Tensor::new(tensor)
    }

    pub fn from_data_device(data: Data<B::Elem, D>, device: B::Device) -> Self {
        let tensor = B::from_data(data, device);
        Tensor::new(tensor)
    }

    pub fn index<const D2: usize>(&self, indexes: [std::ops::Range<usize>; D2]) -> Self {
        Self::new(self.value.index(indexes))
    }

    pub fn index_assign<const D2: usize>(
        &self,
        indexes: [std::ops::Range<usize>; D2],
        values: &Self,
    ) -> Self {
        Self::new(self.value.index_assign(indexes, &values.value))
    }

    pub fn mask_fill(&self, mask: &BoolTensor<B, D>, value: B::Elem) -> Self {
        Self::new(self.value.mask_fill(&mask.value, value))
    }

    pub fn to_full_precision(&self) -> Tensor<B::FullPrecisionBackend, D> {
        Tensor::new(self.value.to_full_precision())
    }

    pub fn from_full_precision(tensor: Tensor<B::FullPrecisionBackend, D>) -> Self {
        let value = B::TensorPrimitive::from_full_precision(tensor.value);
        Tensor::new(value)
    }

    pub fn argmax(&self, dim: usize) -> Tensor<B::IntegerBackend, D> {
        Tensor::new(self.value.argmax(dim))
    }

    pub fn argmin(&self, dim: usize) -> Tensor<B::IntegerBackend, D> {
        Tensor::new(self.value.argmin(dim))
    }

    pub fn cat(tensors: Vec<Self>, dim: usize) -> Self {
        let tensors: Vec<B::TensorPrimitive<D>> =
            tensors.into_iter().map(|a| a.value.clone()).collect();
        let tensors: Vec<&B::TensorPrimitive<D>> = tensors.iter().collect();
        let value = B::TensorPrimitive::cat(tensors, dim);

        Self::new(value)
    }

    pub fn unsqueeze<const D2: usize>(&self) -> Tensor<B, D2> {
        if D2 < D {
            panic!(
                "Can't unsqueeze smaller tensor, got dim {}, expected > {}",
                D2, D
            )
        }

        let mut dims = [1; D2];
        let num_ones = D2 - D;
        let shape = self.shape();

        for i in 0..D {
            dims[i + num_ones] = shape.dims[i];
        }

        let shape = Shape::new(dims);
        self.reshape(shape)
    }

    pub(crate) fn relu(&self) -> Self {
        Self::new(self.value.relu())
    }
}

impl<const D: usize, B> std::ops::Add<Self> for Tensor<B, D>
where
    B: Backend,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tensor::add(&self, &other)
    }
}

impl<E, const D: usize, B> std::ops::Add<E> for Tensor<B, D>
where
    E: Element,
    B: Backend<Elem = E>,
{
    type Output = Self;

    fn add(self, other: E) -> Self {
        Tensor::add_scalar(&self, &other)
    }
}

impl<const D: usize, B> std::ops::Sub<Self> for Tensor<B, D>
where
    B: Backend,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Tensor::sub(&self, &other)
    }
}

impl<E, const D: usize, B> std::ops::Sub<E> for Tensor<B, D>
where
    E: Element,
    B: Backend<Elem = E>,
{
    type Output = Self;

    fn sub(self, other: E) -> Self {
        Tensor::sub_scalar(&self, &other)
    }
}

impl<const D: usize, B> std::ops::Mul<Self> for Tensor<B, D>
where
    B: Backend,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Tensor::mul(&self, &other)
    }
}

impl<E, const D: usize, B> std::ops::Mul<E> for Tensor<B, D>
where
    E: Element,
    B: Backend<Elem = E>,
{
    type Output = Self;

    fn mul(self, other: E) -> Self {
        Tensor::mul_scalar(&self, &other)
    }
}

impl<const D: usize, B> std::ops::Div<Self> for Tensor<B, D>
where
    B: Backend,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Tensor::div(&self, &other)
    }
}

impl<E, const D: usize, B> std::ops::Div<E> for Tensor<B, D>
where
    E: Element,
    B: Backend<Elem = E>,
{
    type Output = Self;

    fn div(self, other: E) -> Self {
        Tensor::div_scalar(&self, &other)
    }
}

impl<const D: usize, B: ADBackend> Tensor<B, D> {
    pub fn backward(&self) -> Gradients {
        B::backward::<D>(&self.value)
    }

    pub fn grad(&self, grads: &Gradients) -> Option<Tensor<B::InnerBackend, D>> {
        B::grad(&self.value, grads).map(|value| Tensor::new(value))
    }

    pub fn inner(&self) -> Tensor<B::InnerBackend, D> {
        Tensor::new(B::inner(&self.value))
    }

    pub fn update(&mut self, other_inner: Tensor<B::InnerBackend, D>) {
        self.value = B::from_inner(other_inner.value);
    }

    pub fn from_inner(inner: Tensor<B::InnerBackend, D>) -> Self {
        Self::new(B::from_inner(inner.value))
    }

    pub fn detach(&self) -> Self {
        Self::from_inner(self.inner())
    }
}
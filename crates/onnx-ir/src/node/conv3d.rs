use crate::ir::Node;
use crate::node::padding::{PaddingConfig3d, padding_config_3d};

/// Configuration for Conv3d operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conv3dConfig {
    /// Input and output channels [in, out].
    pub channels: [usize; 2],
    /// Size of the kernel.
    pub kernel_size: [usize; 3],
    /// Stride of the convolutional kernel.
    pub stride: [usize; 3],
    /// Dilation of the convolutional kernel.
    pub dilation: [usize; 3],
    /// Groups.
    pub groups: usize,
    /// Use bias.
    pub bias: bool,
    /// Padding.
    pub padding: PaddingConfig3d,
}

impl Conv3dConfig {
    /// Create a new configuration for a Conv3d.
    pub fn new(
        channels: [usize; 2],
        kernel_size: [usize; 3],
        stride: [usize; 3],
        dilation: [usize; 3],
        groups: usize,
        bias: bool,
        padding: PaddingConfig3d,
    ) -> Self {
        Self {
            channels,
            kernel_size,
            stride,
            dilation,
            groups,
            bias,
            padding,
        }
    }
}

/// Create a Conv3dConfig from the attributes of the node
pub fn conv3d_config(curr: &Node) -> Conv3dConfig {
    let mut kernel_shape = Vec::new(); // TODO default inferred from weight tensor per spec
    let mut strides = vec![1, 1, 1];
    let mut pads = vec![0, 0, 0, 0, 0, 0];
    let mut dilations = vec![1, 1, 1];
    let mut group: usize = 1;

    let weight_shape = curr.inputs[1]
        .value
        .as_ref()
        .expect("Conv3d: weight tensor must be present")
        .shape
        .clone();

    // check if the bias is present
    let bias = curr.inputs.len() == 3;

    for (key, value) in curr.attrs.iter() {
        match key.as_str() {
            "kernel_shape" => kernel_shape = value.clone().into_i64s(),
            "strides" => strides = value.clone().into_i64s(),
            "pads" => pads = value.clone().into_i64s(),
            "dilations" => dilations = value.clone().into_i64s(),
            "group" => group = value.clone().into_i64() as usize,
            _ => panic!("Unexpected attribute for Conv3d: {key}"),
        }
    }

    // the channels are inverted in the weight tensor
    let channels_in = weight_shape[1] * group;
    let channels_out = weight_shape[0];

    let padding = padding_config_3d(&pads);

    Conv3dConfig::new(
        [channels_in, channels_out],
        [
            kernel_shape[0] as usize,
            kernel_shape[1] as usize,
            kernel_shape[2] as usize,
        ],
        [
            strides[0] as usize,
            strides[1] as usize,
            strides[2] as usize,
        ],
        [
            dilations[0] as usize,
            dilations[1] as usize,
            dilations[2] as usize,
        ],
        group,
        bias,
        padding,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::NodeType;
    use crate::node::test_utils::NodeBuilder;

    fn create_test_node(
        kernel_shape: Vec<i64>,
        strides: Vec<i64>,
        pads: Vec<i64>,
        dilations: Vec<i64>,
        group: i64,
        has_bias: bool,
    ) -> Node {
        // Create weight tensor data (not important for the test)
        let weight_data = vec![0.0; 32];
        let weight_shape = vec![4, 2, 2, 2, 2]; // [output_channels, input_channels/groups, k_d, k_h, k_w]

        // Start building the node with input and weight
        let mut builder = NodeBuilder::new(NodeType::Conv3d, "test_conv3d")
            .input_tensor_f32("data", 5, None)
            .input_tensor_f32_data("weight", weight_data, weight_shape)
            .output_tensor_f32("output", 5, None);

        // Add bias if needed
        if has_bias {
            builder = builder.input_tensor_f32("bias", 1, None);
        }

        // Add attributes
        builder = builder
            .attr_ints("kernel_shape", kernel_shape)
            .attr_ints("strides", strides)
            .attr_ints("pads", pads)
            .attr_ints("dilations", dilations)
            .attr_int("group", group);

        builder.build()
    }

    #[test]
    fn test_conv3d_config_basic() {
        let node = create_test_node(
            vec![2, 2, 2],
            vec![1, 1, 1],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 1],
            1,
            false,
        );
        let config = conv3d_config(&node);

        assert_eq!(config.channels, [2, 4]);
        assert_eq!(config.kernel_size, [2, 2, 2]);
        assert_eq!(config.stride, [1, 1, 1]);
        assert_eq!(config.dilation, [1, 1, 1]);
        assert_eq!(config.groups, 1);
        assert!(!config.bias);
        assert!(matches!(config.padding, PaddingConfig3d::Valid));
    }

    #[test]
    fn test_conv3d_config_with_padding() {
        let node = create_test_node(
            vec![3, 3, 3],
            vec![1, 1, 1],
            vec![1, 1, 1, 1, 1, 1],
            vec![1, 1, 1],
            1,
            false,
        );
        let config = conv3d_config(&node);

        assert_eq!(config.kernel_size, [3, 3, 3]);
        assert!(matches!(config.padding, PaddingConfig3d::Explicit(1, 1, 1)));
    }

    #[test]
    fn test_conv3d_config_with_groups() {
        let node = create_test_node(
            vec![2, 2, 2],
            vec![1, 1, 1],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 1],
            2,
            false,
        );
        let config = conv3d_config(&node);

        assert_eq!(config.groups, 2);
        assert_eq!(config.channels, [4, 4]); // channels_in is adjusted by groups
    }

    #[test]
    fn test_conv3d_config_with_bias() {
        let node = create_test_node(
            vec![2, 2, 2],
            vec![1, 1, 1],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 1],
            1,
            true,
        );
        let config = conv3d_config(&node);

        assert!(config.bias);
    }
}

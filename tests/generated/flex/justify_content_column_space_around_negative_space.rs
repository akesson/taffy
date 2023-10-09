#[test]
fn justify_content_column_space_around_negative_space() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(0.8f32),
                height: taffy::style::Dimension::Length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(0.8f32),
                height: taffy::style::Dimension::Length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node02 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(0.8f32),
                height: taffy::style::Dimension::Length(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Column,
                align_items: Some(taffy::style::AlignItems::Center),
                justify_content: Some(taffy::style::JustifyContent::SpaceAround),
                size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
                ..Default::default()
            },
            &[node00, node01, node02],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(320f32),
                    height: taffy::style::Dimension::Length(320f32),
                },
                border: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(60f32),
                    right: taffy::style::LengthPercentage::Length(60f32),
                    top: taffy::style::LengthPercentage::Length(60f32),
                    bottom: taffy::style::LengthPercentage::Length(60f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 320f32, "width of node {:?}. Expected {}. Actual {}", node, 320f32, size.width);
    assert_eq!(size.height, 320f32, "height of node {:?}. Expected {}. Actual {}", node, 320f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node0, 200f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 60f32, "x of node {:?}. Expected {}. Actual {}", node0, 60f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node0, 60f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 160f32, "width of node {:?}. Expected {}. Actual {}", node00, 160f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node00, 20f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node00, 20f32, location.x);
    assert_eq!(location.y, -25f32, "y of node {:?}. Expected {}. Actual {}", node00, -25f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node01).unwrap();
    assert_eq!(size.width, 160f32, "width of node {:?}. Expected {}. Actual {}", node01, 160f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node01, 20f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node01, 20f32, location.x);
    assert_eq!(location.y, -5f32, "y of node {:?}. Expected {}. Actual {}", node01, -5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node02).unwrap();
    assert_eq!(size.width, 160f32, "width of node {:?}. Expected {}. Actual {}", node02, 160f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node02, 20f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node02, 20f32, location.x);
    assert_eq!(location.y, 15f32, "y of node {:?}. Expected {}. Actual {}", node02, 15f32, location.y);
}
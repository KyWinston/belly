use super::parse;
use crate::compound_style_property;
use crate::ess::ToRectMap;
use crate::style_property;
use bevy::prelude::*;

compound_style_property! {
    #[doc = " Specify element margin by providing values to `Style.margin`"]
    #[doc = " using single `rect-shorthand` syntax:"]
    #[doc = " ```css"]
    #[doc = " margin: 2px 20% 10px auto;"]
    #[doc = " ```"]
    #[doc = " "]
    #[doc = " Margins are used to create space around elements, outside of"]
    #[doc = " any defined borders."]
    #[doc = " <!-- (TODO: link rect-shorthand) -->"]
    MarginProperty("margin", value) {
        let rect = UiRect::try_from(value)?;
        Ok(rect.to_rect_map("margin-"))
    }
}

style_property! {
    #[doc = " Specify element left margin by providing value to `Style.margin.left`"]
    #[doc = " using `val` syntax:"]
    #[doc = " ```css"]
    #[doc = " margin-left: 5px;"]
    #[doc = " ```"]
    #[doc = " "]
    #[doc = " Margins are used to create space around elements, outside of"]
    #[doc = " any defined borders."]
    #[doc = " <!-- (TODO: link val) -->"]
    MarginLeftProperty("margin-left") {
        Default = "undefined";
        Item = Val;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::ValParser;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.margin.left != value {
                style.margin.left = *value;
            }
        };
    }
}
// impl_style_single_value!("margin-right", MarginRightProperty, Val, val, margin.right);
style_property! {
    #[doc = " Specify element right margin by providing value to `Style.margin.right`"]
    #[doc = " using `val` syntax:"]
    #[doc = " ```css"]
    #[doc = " margin-right: 5px;"]
    #[doc = " ```"]
    #[doc = " "]
    #[doc = " Margins are used to create space around elements, outside of"]
    #[doc = " any defined borders."]
    #[doc = " <!-- (TODO: link val) -->"]
    MarginRightProperty("margin-right") {
        Default = "undefined";
        Item = Val;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::ValParser;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.margin.right != value {
                style.margin.right = *value;
            }
        };
    }
}
// impl_style_single_value!("margin-top", MarginTopProperty, Val, val, margin.top);
style_property! {
    #[doc = " Specify element top margin by providing value to `Style.margin.top`"]
    #[doc = " using `val` syntax:"]
    #[doc = " ```css"]
    #[doc = " margin-top: 5px;"]
    #[doc = " ```"]
    #[doc = " "]
    #[doc = " Margins are used to create space around elements, outside of"]
    #[doc = " any defined borders."]
    #[doc = " <!-- (TODO: link val) -->"]
    MarginTopProperty("margin-top") {
        Default = "undefined";
        Item = Val;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::ValParser;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.margin.top != value {
                style.margin.top = *value;
            }
        };
    }
}

style_property! {
    #[doc = " Specify element bottom margin by providing value to `Style.margin.bottom`"]
    #[doc = " using `val` syntax:"]
    #[doc = " ```css"]
    #[doc = " margin-bottom: 5px;"]
    #[doc = " ```"]
    #[doc = " "]
    #[doc = " Margins are used to create space around elements, outside of"]
    #[doc = " any defined borders."]
    #[doc = " <!-- (TODO: link val) -->"]
    MarginBottomProperty("margin-bottom") {
        Default = "undefined";
        Item = Val;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::ValParser;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.margin.bottom != value {
                style.margin.bottom = *value;
            }
        };
    }
}

compound_style_property! {
    #[doc = " Specify element padding by providing values to `Style.padding`"]
    #[doc = " using single `rect-shorthand` syntax:"]
    #[doc = " ```css"]
    #[doc = " padding: 2px 20% 10px auto;"]
    #[doc = " ```"]
    #[doc = " "]
    #[doc = " Padding is used to create space around an element's content, inside of"]
    #[doc = " any defined borders."]
    #[doc = " <!-- (TODO: link rect-shorthand) -->"]
    PaddingProperty("padding", value) {
        let rect = UiRect::try_from(value)?;
        Ok(rect.to_rect_map("padding-"))
    }
}

style_property! {
    #[doc = " Specify element left padding by providing value to `Style.padding.left`"]
    #[doc = " using `val` syntax:"]
    #[doc = " ```css"]
    #[doc = " padding-left: 5px;"]
    #[doc = " ```"]
    #[doc = " "]
    #[doc = " Padding is used to create space around an element's content, inside of"]
    #[doc = " any defined borders."]
    #[doc = " <!-- (TODO: link val) -->"]
    PaddingLeftProperty("padding-left") {
        Default = "undefined";
        Item = Val;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::ValParser;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.padding.left != value {
                style.padding.left = *value;
            }
        };
    }
}

style_property! {
    #[doc = " Specify element right padding by providing value to `Style.padding.right`"]
    #[doc = " using `val` syntax:"]
    #[doc = " ```css"]
    #[doc = " padding-right: 5px;"]
    #[doc = " ```"]
    #[doc = " "]
    #[doc = " Padding is used to create space around an element's content, inside of"]
    #[doc = " any defined borders."]
    #[doc = " <!-- (TODO: link val) -->"]
    PaddingRightProperty("padding-right") {
        Default = "undefined";
        Item = Val;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::ValParser;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.padding.right != value {
                style.padding.right = *value;
            }
        };
    }
}

style_property! {
    #[doc = " Specify element top padding by providing value to `Style.padding.top`"]
    #[doc = " using `val` syntax:"]
    #[doc = " ```css"]
    #[doc = " padding-top: 5px;"]
    #[doc = " ```"]
    #[doc = " "]
    #[doc = " Padding is used to create space around an element's content, inside of"]
    #[doc = " any defined borders."]
    #[doc = " <!-- (TODO: link val) -->"]
    PaddingTopProperty("padding-top") {
        Default = "undefined";
        Item = Val;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::ValParser;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.padding.top != value {
                style.padding.top = *value;
            }
        };
    }
}

style_property! {
    #[doc = " Specify element bottom padding by providing value to `Style.padding.bottom`"]
    #[doc = " using `val` syntax:"]
    #[doc = " ```css"]
    #[doc = " padding-bottom: 5px;"]
    #[doc = " ```"]
    #[doc = " "]
    #[doc = " Padding is used to create space around an element's content, inside of"]
    #[doc = " any defined borders."]
    #[doc = " <!-- (TODO: link val) -->"]
    PaddingBottomProperty("padding-bottom") {
        Default = "undefined";
        Item = Val;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::ValParser;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.padding.bottom != value {
                style.padding.bottom = *value;
            }
        };
    }
}

compound_style_property! {
    #[doc = " Specify element border width by providing values to `Style.border`"]
    #[doc = " using single `rect-shorthand` syntax:"]
    #[doc = " ```css"]
    #[doc = " border-width: 2px 20% 10px auto;"]
    #[doc = " ```"]
    #[doc = " "]
    #[doc = " The `border-width` property specifies the width of the four borders."]
    #[doc = " <!-- (TODO: link rect-shorthand) -->"]
    BorderProperty("border-width", value) {
        let rect = UiRect::try_from(value)?;
        Ok(rect.to_rect_map("border-width-"))
    }
}

style_property! {
    #[doc = " Specify element left border width by providing value to `Style.border.left`"]
    #[doc = " using `val` syntax:"]
    #[doc = " ```css"]
    #[doc = " border-width-left: 5px;"]
    #[doc = " ```"]
    #[doc = " <!-- (TODO: link val) -->"]
    BorderLeftProperty("border-width-left") {
        Default = "undefined";
        Item = Val;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::ValParser;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.border.left != value {
                style.border.left = *value;
            }
        };
    }
}

style_property! {
    #[doc = " Specify element right border width by providing value to `Style.border.right`"]
    #[doc = " using `val` syntax:"]
    #[doc = " ```css"]
    #[doc = " border-width-right: 5px;"]
    #[doc = " ```"]
    #[doc = " <!-- (TODO: link val) -->"]
    BorderRightProperty("border-width-right") {
        Default = "undefined";
        Item = Val;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::ValParser;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.border.right != value {
                style.border.right = *value;
            }
        };
    }
}

style_property! {
    #[doc = " Specify element top border width by providing value to `Style.border.top`"]
    #[doc = " using `val` syntax:"]
    #[doc = " ```css"]
    #[doc = " border-width-top: 5px;"]
    #[doc = " ```"]
    #[doc = " <!-- (TODO: link val) -->"]
    BorderTopProperty("border-width-top") {
        Default = "undefined";
        Item = Val;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::ValParser;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.border.top != value {
                style.border.top = *value;
            }
        };
    }
}

style_property! {
    #[doc = " Specify element bottom border width by providing value to `Style.border.bottom`"]
    #[doc = " using `val` syntax:"]
    #[doc = " ```css"]
    #[doc = " border-width-bottom: 5px;"]
    #[doc = " ```"]
    #[doc = " <!-- (TODO: link val) -->"]
    BorderBottomProperty("border-width-bottom") {
        Default = "undefined";
        Item = Val;
        Components = &'static mut Style;
        Filters = With<Node>;
        Parser = parse::ValParser;
        Apply = |value, style, _assets, _commands, _entity| {
            if &style.border.bottom != value {
                style.border.bottom = *value;
            }
        };
    }
}
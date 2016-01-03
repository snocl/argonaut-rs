macro_rules! tag_structs {
    ( $( $tag:ident: $func:ident -> $res:ty ),* ) => {
        $(
            /// The handle for a specific argument added to an argument parser.
            #[derive(Debug, Clone, PartialEq)]
            pub struct $tag {
                id: Id
            }
            
            impl $tag {
                /// Gets the value of this argument in the parsed arguments.
                pub fn get<'a>(&self, arguments: &'a ParsedArguments<'a>) -> $res {
                    arguments.$func(&self.id)
                }
            }
        )*
    }
}

macro_rules! argument_type_structs {
    (
        common: $common_struct:ident {
            name: $common_name_type:ty,
            help: $common_help_type:ty
        }
        
        $(
            $sub_struct:ident { 
                $( $sub_struct_member:ident : $sub_struct_memtype:ty ),*
            }
        
            tag: $sub_struct_add_func:ident -> $sub_struct_tag_type:ty,
            constructors: {
            $(
                $sub_constructor:ident ( 
                    $(
                        $sub_parameter:ident : $sub_parameter_type:ty 
                    ),* 
                ) -> {
                    $(
                        $sub_constructor_argument:ident : 
                        $sub_constructor_value:expr
                    ),*
                }  
            )*
            }
        )*
    ) => {
        $(
            /// A specialized argument.
            #[derive(Debug)]
            pub struct $sub_struct<'a> {
                name: $common_name_type,
                help: $common_help_type,
                $( $sub_struct_member: $sub_struct_memtype ),*
            }
            
            impl<'a> $sub_struct<'a> {
                /// Adds the argument to the parser and returns a handle to it.
                pub fn add_to<'t>(&'a self, parser: &mut ArgumentParser<'a, 't>)
                        -> Result<$sub_struct_tag_type, String> {
                    parser.$sub_struct_add_func(self)
                }
            }
        )*
        
        /// A common argument.
        #[derive(Debug)]
        pub struct $common_struct<'a> {
            name: $common_name_type,
            help: $common_help_type,
        }
        
        impl<'a> $common_struct<'a> {
            $($(
                pub fn $sub_constructor(
                    self, $( $sub_parameter : $sub_parameter_type),* 
                ) -> $sub_struct<'a> {
                    $sub_struct {
                        name: self.name,
                        help: self.help,
                        $( 
                            $sub_constructor_argument : $sub_constructor_value 
                        ),*
                    }
                }
            )*)*
        }
    }
}

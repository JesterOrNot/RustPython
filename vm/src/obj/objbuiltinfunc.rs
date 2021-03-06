use std::fmt;

use crate::descriptor::PyBuiltinDescriptor;
use crate::function::{OptionalArg, PyFuncArgs, PyNativeFunc};
use crate::obj::objtype::PyClassRef;
use crate::pyobject::{
    IdProtocol, PyClassImpl, PyContext, PyObjectRef, PyRef, PyResult, PyValue, TypeProtocol,
};
use crate::vm::VirtualMachine;

#[pyclass]
pub struct PyBuiltinFunction {
    value: PyNativeFunc,
}

impl PyValue for PyBuiltinFunction {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.builtin_function_or_method_type()
    }
}

impl fmt::Debug for PyBuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "builtin function")
    }
}

impl PyBuiltinFunction {
    pub fn new(value: PyNativeFunc) -> Self {
        Self { value }
    }

    pub fn as_func(&self) -> &PyNativeFunc {
        &self.value
    }
}

#[pyimpl]
impl PyBuiltinFunction {
    #[pymethod(name = "__call__")]
    pub fn call(&self, args: PyFuncArgs, vm: &VirtualMachine) -> PyResult {
        (self.value)(vm, args)
    }
}

#[pyclass]
pub struct PyBuiltinMethod {
    function: PyBuiltinFunction,
}

impl PyValue for PyBuiltinMethod {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.method_descriptor_type()
    }
}

impl fmt::Debug for PyBuiltinMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "method descriptor")
    }
}

impl PyBuiltinMethod {
    pub fn new(value: PyNativeFunc) -> Self {
        Self {
            function: PyBuiltinFunction { value },
        }
    }

    pub fn as_func(&self) -> &PyNativeFunc {
        &self.function.value
    }
}

impl PyBuiltinDescriptor for PyBuiltinMethod {
    fn get(
        zelf: PyRef<Self>,
        obj: PyObjectRef,
        cls: OptionalArg<PyObjectRef>,
        vm: &VirtualMachine,
    ) -> PyResult {
        if obj.is(&vm.get_none()) && !Self::_cls_is(&cls, &obj.class()) {
            Ok(zelf.into_object())
        } else {
            Ok(vm.ctx.new_bound_method(zelf.into_object(), obj))
        }
    }
}

#[pyimpl]
impl PyBuiltinMethod {
    #[pymethod(name = "__call__")]
    pub fn call(&self, args: PyFuncArgs, vm: &VirtualMachine) -> PyResult {
        self.function.call(args, vm)
    }
}

pub fn init(context: &PyContext) {
    PyBuiltinFunction::extend_class(context, &context.types.builtin_function_or_method_type);
    PyBuiltinMethod::extend_class(context, &context.types.method_descriptor_type);
    extend_class!(context, context.types.method_descriptor_type, {
        "__get__" => context.new_method(PyBuiltinMethod::get),
        (slot descr_get) => PyBuiltinMethod::get,
    });
}

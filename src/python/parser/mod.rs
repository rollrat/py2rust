//! This code is a part of py2rust project.
//! Copyright (C) 2019. py2rust developers. Licensed under the MIT Licence.

use std::collections::HashMap;

trait PythonValue {
  fn line() -> u32;
  fn column() -> u32;
  fn parent() -> &PythonValue;
  fn name() -> String;
}

struct PythonDebugInfo {
  line: u32,
  column: u32,
  moduleId: u32,
}

struct PythonValue {
  parent: &PythonValue,
  name: String
}

struct PythonImport {

}

struct PythonModule {
  moduleFileName: String,
  refModules: HashMap<String, &PythonModule>,
  refImports: [&PythonImport]
}

struct PythonArgument {

}
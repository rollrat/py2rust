//! This code is a part of py2rust project.
//! Copyright (C) 2019. py2rust developers. Licensed under the MIT Licence.

use std::collections::HashMap;

struct PythonDebugInfo {
  line: u32,
  column: u32,
  moduleId: u32,
}

impl PythonDebugInfo {
  fn new(line: u32, column: u32, moduleId: u32) -> Self {
    PythonDebugInfo { line, column, moduleId }
  }
  fn line(&self) -> u32 { self.line }
  fn column(&self) -> u32 { self.column }
  fn moduleId(&self) -> u32 { self.moduleId }
}

struct PythonValue<'a> {
  debugInfo: PythonDebugInfo,
  parent: &'a PythonValue<'a>,
  name: String
}

impl PythonValue<'_> {
  fn new<'a>(line: u32, column: u32, moduleId: u32, parent: &'a PythonValue, name: String) -> Self {
    PythonValue { debugInfo: PythonDebugInfo{line, column, moduleId}, parent, name }
  }
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
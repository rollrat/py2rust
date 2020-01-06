//! This code is a part of py2rust project.
//! Copyright (C) 2019. py2rust developers. Licensed under the MIT Licence.

mod parser;

enum PythonPrimitiveType {
  None,
  Object,
  Int,
  Float,
  String,
  Unicode,
  List,
  Dictionary,
  Tuple,
  Lambda,
  Type,
}


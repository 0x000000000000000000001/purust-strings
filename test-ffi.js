import { findFfiFileImpl } from '../../purescript-backend-optimizer/src/PureScript/Backend/Optimizer/FfiSupport.js';
const find = findFfiFileImpl('.rs')([])('../')('Data.Show')(null);
console.log("Result:", find());

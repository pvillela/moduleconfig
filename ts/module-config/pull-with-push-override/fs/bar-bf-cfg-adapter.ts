/*
 * Copyright © 2022 Paulo Villela. All rights reserved.
 * Use of this source code is governed by the MIT license
 * that can be found in the LICENSE file.
 */

import { AppCfgInfo } from "../config/app-cfg";
import { BarBfCfgInfo } from "./bar-bf";

export function barBfCfgAdapter(appCfg: AppCfgInfo): BarBfCfgInfo {
  return {
    z: appCfg.y
  };
}

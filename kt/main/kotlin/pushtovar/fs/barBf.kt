/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package pushtovar.fs

import pushtovar.fwk.CfgSrc

data class BarBfCfgInfo(
	val z: Int
)

val barBfCfgSrc = CfgSrc<BarBfCfgInfo>()

fun barBf() {
	println(barBfCfgSrc().z)
}

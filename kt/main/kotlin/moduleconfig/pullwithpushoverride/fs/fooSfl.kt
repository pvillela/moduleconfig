/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package tryout.moduleconfig.pullwithpushoverride.fs

import tryout.moduleconfig.pullwithpushoverride.config.CfgSrc
import tryout.moduleconfig.pullwithpushoverride.config.makeCfgSrc

data class FooSflCfgInfo(
	val x: String
)

var fooSflCfgSrc: CfgSrc<FooSflCfgInfo> = makeCfgSrc(::fooSflCfgAdapter)

fun fooSfl() {
	println(fooSflCfgSrc().x)
	barBf()
}

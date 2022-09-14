/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package tryout.moduleconfig.pullwithpushoverride.run

import tryout.moduleconfig.pullwithpushoverride.fs.*

fun main() {
	fooSflCfgSrc = { FooSflCfgInfo("foo") }

	barBfCfgSrc = { BarBfCfgInfo(99) }

	fooSfl()
}

/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package main

import (
	"github.com/pvillela/module-config/go/module-config/push-to-var/fs"
	_ "github.com/pvillela/module-config/go/module-config/push-to-var/startup"
)

func main() {
	fs.FooSfl()
}

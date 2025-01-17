/* Copyright (C) 2020 Open Information Security Foundation
 *
 * You can copy, redistribute or modify this Program under the terms of
 * the GNU General Public License version 2 as published by the Free
 * Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * version 2 along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301, USA.
 */

extern crate proc_macro;

use proc_macro::TokenStream;

mod applayerevent;

/// The `AppLayerEvent` derive macro generates a `AppLayerEvent` trait
/// implementation for enums that define AppLayerEvents.
///
/// Example usage (DNS app-layer events):
///
/// #[derive(AppLayerEvent)]
/// enum {
///     MalformedData,
///     NotRequest,
///     NotResponse,
///     ZFlagSet,
/// }
///
/// The enum variants must follow the naming convention of OneTwoThree
/// for proper conversion to the name used in rules (one_tow_three).
#[proc_macro_derive(AppLayerEvent)]
pub fn derive_app_layer_event(input: TokenStream) -> TokenStream {
    applayerevent::derive_app_layer_event(input)
}

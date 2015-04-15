// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! An interface for scrollable widgets

use cast::GTK_SCROLLABLE;
use ffi;

/// GtkScrollable — An interface for scrollable widgets
pub trait ScrollableTrait: ::WidgetTrait {
    fn get_hadjustment(&self) -> ::Adjustment {
        unsafe {
            ::Adjustment::wrap_pointer(ffi::gtk_scrollable_get_hadjustment(GTK_SCROLLABLE(self.unwrap_widget())))
        }
    }

    fn set_hadjustment(&self, hadjustment: ::Adjustment) {
        unsafe {
            ffi::gtk_scrollable_set_hadjustment(GTK_SCROLLABLE(self.unwrap_widget()),
                                                hadjustment.unwrap_pointer())
        }
    }

    fn get_vadjustment(&self) -> ::Adjustment {
        unsafe {
            ::Adjustment::wrap_pointer(ffi::gtk_scrollable_get_vadjustment(GTK_SCROLLABLE(self.unwrap_widget())))
        }
    }

    fn set_vadjustment(&self, vadjustment: ::Adjustment) {
        unsafe {
            ffi::gtk_scrollable_set_vadjustment(GTK_SCROLLABLE(self.unwrap_widget()),
                                                vadjustment.unwrap_pointer())
        }
    }

    fn get_hscroll_policy(&self) -> ::ScrollablePolicy {
        unsafe {
            ffi::gtk_scrollable_get_hscroll_policy(GTK_SCROLLABLE(self.unwrap_widget()))
        }
    }

    fn set_hscroll_policy(&self, policy: ::ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_hscroll_policy(GTK_SCROLLABLE(self.unwrap_widget()),
                                                   policy)
        }
    }

    fn get_vscroll_policy(&self) -> ::ScrollablePolicy {
        unsafe {
            ffi::gtk_scrollable_get_vscroll_policy(GTK_SCROLLABLE(self.unwrap_widget()))
        }
    }

    fn set_vscroll_policy(&self, policy: ::ScrollablePolicy) {
        unsafe {
            ffi::gtk_scrollable_set_vscroll_policy(GTK_SCROLLABLE(self.unwrap_widget()),
                                                   policy)
        }
    }
}

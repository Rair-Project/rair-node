/*
 * rair-node: Node bindings for Rair-core
 * Copyright (C) 2019  Oddcoder
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use neon::prelude::*;
use rio::*;

declare_types! {
  pub class JSRIO for RIO {
    init(mut _cx) {
      let io = RIO::new();
      Ok(io)
    }

    method open(mut cx) {
      let mut this = cx.this();
      let uri :String = cx.argument::<JsString>(0)?.value();
      let mode_string :String = cx.argument::<JsString>(1)?.value();
      let mut mode: IoMode = Default::default();
      for c in mode_string.chars() {
        match c {
            'R' => mode |= IoMode::READ,
            'W' => mode |= IoMode::WRITE,
            'X' => mode |= IoMode::EXECUTE,
            _ => panic!("RIO.open: Invalid Argument")
        }
      }
      let handle = {
        let guard = cx.lock();
        let mut io = this.borrow_mut(&guard);
       io.open(&uri, mode).unwrap_or_else(|e| panic! (e.to_string()))
      };
      return Ok(cx.number(handle as f64).upcast());
    }
    method open_at(mut cx) {
      let mut this = cx.this();
      let uri :String = cx.argument::<JsString>(0)?.value();
      let mode_string :String = cx.argument::<JsString>(1)?.value();
      let mut mode: IoMode = Default::default();
      let location :u64 = cx.argument::<JsNumber>(2)?.value() as u64;
      for c in mode_string.chars() {
        match c {
          'R' => mode |= IoMode::READ,
          'W' => mode |= IoMode::WRITE,
          'X' => mode |= IoMode::EXECUTE,
          _ => panic!("RIO.open: Invalid Argument")
        }
      }
      let handle = {
        let guard = cx.lock();
        let mut io = this.borrow_mut(&guard);
       io.open_at(&uri, mode, location).unwrap_or_else(|e| panic! (e.to_string()))
      };
      return Ok(cx.number(handle as f64).upcast());
    }
    method close(mut cx) {
      let ret = cx.null().upcast();
      let mut this = cx.this();
      let hndl : u64 = cx.argument::<JsNumber>(2)?.value() as u64;
      let guard = cx.lock();
      let mut io = this.borrow_mut(&guard); 
      io.close(hndl).unwrap_or_else(|e| panic! (e.to_string()));
      return Ok(ret);
    }
    method close_all(mut cx) {
        let ret = cx.null().upcast();
        let mut this = cx.this();
        let guard = cx.lock();
        let mut io = this.borrow_mut(&guard); 
        io.close_all();
        return Ok(ret);
    }
    method pread(mut cx) {
      let ret = cx.null().upcast();
      let mut this = cx.this();
      let addr = cx.argument::<JsNumber>(0)?.value() as u64;
      let mut buffer :Handle<JsBuffer> = cx.argument(1)?;
      let guard = cx.lock();
      let mut io = this.borrow_mut(&guard);
      cx.borrow_mut(&mut buffer, |data| {
        let slice = data.as_mut_slice::<u8>();
        io.pread(addr, slice).unwrap_or_else(|e| panic! (e.to_string()))
      });
      return Ok(ret);
    }
    
  }
}
register_module!(mut m, { m.export_class::<JSRIO>("RIO") });

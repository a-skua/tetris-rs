// wasm/wasm.js
var wasm;
var heap = new Array(128).fill(void 0);
heap.push(void 0, null, true, false);
function getObject(idx) {
  return heap[idx];
}
var heap_next = heap.length;
function dropObject(idx) {
  if (idx < 132)
    return;
  heap[idx] = heap_next;
  heap_next = idx;
}
function takeObject(idx) {
  const ret = getObject(idx);
  dropObject(idx);
  return ret;
}
var cachedTextDecoder = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
var cachedUint8Memory0 = null;
function getUint8Memory0() {
  if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
    cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8Memory0;
}
function getStringFromWasm0(ptr, len) {
  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
function addHeapObject(obj) {
  if (heap_next === heap.length)
    heap.push(heap.length + 1);
  const idx = heap_next;
  heap_next = heap[idx];
  heap[idx] = obj;
  return idx;
}
var cachedInt32Memory0 = null;
function getInt32Memory0() {
  if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
    cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
  }
  return cachedInt32Memory0;
}
function handleError(f, args) {
  try {
    return f.apply(this, args);
  } catch (e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
  }
}
function getArrayU8FromWasm0(ptr, len) {
  return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
var JsState = Object.freeze({ Empty: 0, "0": "Empty", Block: 1, "1": "Block" });
var JsInput = Object.freeze({ MoveLeft: 0, "0": "MoveLeft", MoveRight: 1, "1": "MoveRight", MoveTop: 2, "2": "MoveTop", MoveBottom: 3, "3": "MoveBottom", RotateLeft: 4, "4": "RotateLeft", RotateRight: 5, "5": "RotateRight" });
var Tetris = class {
  static __wrap(ptr) {
    const obj = Object.create(Tetris.prototype);
    obj.ptr = ptr;
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.ptr;
    this.ptr = 0;
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_tetris_free(ptr);
  }
  /**
  * @returns {Tetris}
  */
  static new() {
    const ret = wasm.tetris_new();
    return Tetris.__wrap(ret);
  }
  /**
  * @returns {number}
  */
  deside() {
    const ret = wasm.tetris_deside(this.ptr);
    return ret >>> 0;
  }
  /**
  * @param {number} input
  */
  input(input) {
    wasm.tetris_input(this.ptr, input);
  }
  /**
  * @returns {string}
  */
  to_string() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.tetris_to_string(retptr, this.ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_free(r0, r1);
    }
  }
  /**
  * @returns {number}
  */
  size_x() {
    const ret = wasm.tetris_size_x(this.ptr);
    return ret >>> 0;
  }
  /**
  * @returns {number}
  */
  size_y() {
    const ret = wasm.tetris_size_y(this.ptr);
    return ret >>> 0;
  }
  /**
  * @param {number} x
  * @param {number} y
  * @returns {number}
  */
  state(x, y) {
    const ret = wasm.tetris_state(this.ptr, x, y);
    return ret >>> 0;
  }
};
async function load(module2, imports) {
  if (typeof Response === "function" && module2 instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming === "function") {
      try {
        return await WebAssembly.instantiateStreaming(module2, imports);
      } catch (e) {
        if (module2.headers.get("Content-Type") != "application/wasm") {
          console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
        } else {
          throw e;
        }
      }
    }
    const bytes = await module2.arrayBuffer();
    return await WebAssembly.instantiate(bytes, imports);
  } else {
    const instance = await WebAssembly.instantiate(module2, imports);
    if (instance instanceof WebAssembly.Instance) {
      return { instance, module: module2 };
    } else {
      return instance;
    }
  }
}
function getImports() {
  const imports = {};
  imports.wbg = {};
  imports.wbg.__wbg_randomFillSync_6894564c2c334c42 = function() {
    return handleError(function(arg0, arg1, arg2) {
      getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    }, arguments);
  };
  imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
  };
  imports.wbg.__wbg_getRandomValues_805f1c3d65988a5a = function() {
    return handleError(function(arg0, arg1) {
      getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments);
  };
  imports.wbg.__wbg_crypto_e1d53a1d73fb10b8 = function(arg0) {
    const ret = getObject(arg0).crypto;
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_is_object = function(arg0) {
    const val = getObject(arg0);
    const ret = typeof val === "object" && val !== null;
    return ret;
  };
  imports.wbg.__wbg_process_038c26bf42b093f8 = function(arg0) {
    const ret = getObject(arg0).process;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_versions_ab37218d2f0b24a8 = function(arg0) {
    const ret = getObject(arg0).versions;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_node_080f4b19d15bc1fe = function(arg0) {
    const ret = getObject(arg0).node;
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_is_string = function(arg0) {
    const ret = typeof getObject(arg0) === "string";
    return ret;
  };
  imports.wbg.__wbg_msCrypto_6e7d3e1f92610cbb = function(arg0) {
    const ret = getObject(arg0).msCrypto;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_require_78a3dcfbdba9cbce = function() {
    return handleError(function() {
      const ret = module.require;
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbindgen_is_function = function(arg0) {
    const ret = typeof getObject(arg0) === "function";
    return ret;
  };
  imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_newnoargs_2b8b6bd7753c76ba = function(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_call_95d1ea488d03e4e8 = function() {
    return handleError(function(arg0, arg1) {
      const ret = getObject(arg0).call(getObject(arg1));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_self_e7c1f827057f6584 = function() {
    return handleError(function() {
      const ret = self.self;
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_window_a09ec664e14b1b81 = function() {
    return handleError(function() {
      const ret = window.window;
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_globalThis_87cbb8506fecf3a9 = function() {
    return handleError(function() {
      const ret = globalThis.globalThis;
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_global_c85a9259e621f3db = function() {
    return handleError(function() {
      const ret = global.global;
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbindgen_is_undefined = function(arg0) {
    const ret = getObject(arg0) === void 0;
    return ret;
  };
  imports.wbg.__wbg_call_9495de66fdbe016b = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_buffer_cf65c07de34b9a08 = function(arg0) {
    const ret = getObject(arg0).buffer;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_new_537b7341ce90bb31 = function(arg0) {
    const ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_set_17499e8aa4003ebd = function(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
  };
  imports.wbg.__wbg_length_27a2afe8ab42b09f = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
  };
  imports.wbg.__wbg_newwithlength_b56c882b57805732 = function(arg0) {
    const ret = new Uint8Array(arg0 >>> 0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_subarray_7526649b91a252a6 = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
  };
  imports.wbg.__wbindgen_memory = function() {
    const ret = wasm.memory;
    return addHeapObject(ret);
  };
  return imports;
}
function initMemory(imports, maybe_memory) {
}
function finalizeInit(instance, module2) {
  wasm = instance.exports;
  init.__wbindgen_wasm_module = module2;
  cachedInt32Memory0 = null;
  cachedUint8Memory0 = null;
  return wasm;
}
async function init(input) {
  if (typeof input === "undefined") {
    input = new URL("wasm_bg.wasm", import.meta.url);
  }
  const imports = getImports();
  if (typeof input === "string" || typeof Request === "function" && input instanceof Request || typeof URL === "function" && input instanceof URL) {
    input = fetch(input);
  }
  initMemory(imports);
  const { instance, module: module2 } = await load(await input, imports);
  return finalizeInit(instance, module2);
}
var wasm_default = init;

// main.ts
var block_size = 20;
var fps = 30;
var rendering = (tetris, ctx) => {
  ctx.clearRect(
    0,
    0,
    tetris.size_x() * block_size,
    tetris.size_y() * block_size
  );
  for (let y = 0; y < tetris.size_y(); y++) {
    for (let x = 0; x < tetris.size_x(); x++) {
      const state = tetris.state(x, y);
      switch (state) {
        case JsState.Block:
          ctx.fillRect(x * block_size, y * block_size, block_size, block_size);
          break;
        default:
          ctx.fillRect(
            x * block_size + block_size / 2,
            y * block_size + block_size / 2,
            1,
            1
          );
      }
    }
  }
  console.log(tetris.to_string());
};
var createPointElement = (point) => document.createTextNode(`point: ${point}`);
var main_default = () => wasm_default().then(() => {
  const tetris = Tetris.new();
  const canvas = document.createElement("canvas");
  canvas.width = tetris.size_x() * block_size;
  canvas.height = tetris.size_y() * block_size;
  canvas.style.setProperty("border", "solid");
  document.body.appendChild(canvas);
  const info = document.createElement("div");
  info.appendChild(
    document.createTextNode("\u2190: h / \u2193: j or k / \u2192: l / rotate: p, n, [ or ]")
  );
  info.appendChild(document.createElement("br"));
  document.body.appendChild(info);
  let count = 0;
  info.appendChild(createPointElement(count));
  setInterval(
    () => rendering(tetris, canvas.getContext("2d")),
    1e3 / fps
  );
  setInterval(() => {
    count += tetris.deside();
    info.removeChild(info.lastChild);
    info.appendChild(createPointElement(count));
  }, 1e3);
  self.window.addEventListener("keydown", (e) => {
    console.log(e.key);
    switch (e.key) {
      case "ArrowLeft":
      case "h":
        tetris.input(JsInput.MoveLeft);
        break;
      case "ArrowDown":
      case "j":
        tetris.input(JsInput.MoveBottom);
        break;
      case "ArrowUp":
      case "k":
        tetris.input(JsInput.MoveTop);
        break;
      case "ArrowRight":
      case "l":
        tetris.input(JsInput.MoveRight);
        break;
      case "[":
      case "p":
        tetris.input(JsInput.RotateLeft);
        break;
      case "]":
      case "n":
        tetris.input(JsInput.RotateRight);
        break;
    }
  });
});
export {
  main_default as default
};

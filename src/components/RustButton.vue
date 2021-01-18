<template>
    <div>
        <h1>{{ msg }}</h1>
        <button @click="call_rust">
            Click Me!
        </button>
    </div>
</template>
<script>
import { promisified } from 'tauri/api/tauri'
export default {
  name: 'RustButton',
  methods: {
    call_rust() {
        promisified({
            cmd: 'getResponse',
            payload: {
                state: 1
            }
        }).then(response => {
            // do something with the Ok() response
            const { message } = response;
            this.msg_set = message;
        }).catch(error => {
            // do something with the Err() response string
            console.log(error);
        })
        console.log('Rust invoked with Promisified!');
    }
  },
  data: function () {
    return {
      msg: ''
    }
  },
  computed: {
    msg_set: {
        set: function (newValue) {
        this.msg = newValue
        },
        get: function () {
            return this.msg
        },
    }
  }
};
</script>
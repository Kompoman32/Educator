<template>
  <form @submit.prevent="addClass">
    <div class="form-group">
      <label class="control-label">Name:</label>
      <input v-model="nameStudent" type="text" class="form-control" placeholder="Enter Student" maxlength="260" required>
      <input v-model="nameClass" type="text" class="form-control" placeholder="Enter Class" maxlength="260" required>
      <button type="submit" class="btn btn-lg btn-block btn-primary">addTask</button>
    </div>
  </form>
</template>

<script>
  import Tab from '../components/Tab.vue'
  import Tabs from '../components/Tabs.vue'
  import Modal from '../components/Modal.vue'
  import Spinner from '../components/Spinner.vue'

  module.exports = {
    components: {
      Tab,
      Tabs,
      Modal,
      Spinner
    },
    data() {
      return {
        name: '',
        secretKey: '',
        keyPair: {},
        isModalVisible: false,
        isSpinnerVisible: false
      }
    },
    methods: {
            async addClass() {
        if (!this.nameStudent!='' && !this.nameClass!='') {
          return this.$notify('error', 'Введите ид класса и студента')
        }
        try {
            await this.$blockchain.addClass(this.keyPair, this.nameStudent, this.nameClass)
            this.name = ''
            this.isSpinnerVisible = false
            this.isModalVisible = true

        } catch (error) {
          this.isSpinnerVisible = false
          this.$notify('error', error.toString())
        }

            },
    }
  }
</script>

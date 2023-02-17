pipeline {
  agent any
  stages {
    stage('check deps') {
      steps {
        sh 'cargo --version'
      }
    }
    stage('compile') {
      steps {
        sh 'cargo build'
      }
    }
    stage('run with cargo') {
      steps {
        sh 'cargo run'
      }
    }
  }
}

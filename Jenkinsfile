pipeline {
    agent {
        table 'rust'
    }

    stages {
        stage('Build') {
            steps {
                sh "cargo build"
            }
        }
        stage('Test') {
            steps {
                sh "cargo test"
            }
        }
        stage('Clippy') {
            steps {
                sh "cargo +nightly clippy --all"
            }
        }
        stage('Rustfmt') {
            steps {
                sh "cargo +nightly fmt --all -- --write-mode diff"
            }
        }
        stage('Doc') {
            steps {
                sh "cargo doc"
                step([$class: 'JavadocArchiver',
                      javadocDir: 'target/doc',
                      keepAll: false])
            }
        }
    }
}
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
	}

	stages {
		stage('Test') {
			steps {
				sh "cargo build"
			}
		}
	}

	stage('Clippy') {
            steps {
                sh "cargo +nightly clippy --all"
            }
    }
}
node('node') {


    currentBuild.result = "SUCCESS"

    try {

       stage('Checkout 123'){

          checkout scm
       }

       stage('Test 456'){
         print "Environment will be : TESTING"
        
         sh 'cargo build'
       }
    }
    catch (err) {

        currentBuild.result = "FAILURE"
        throw err
    }

}

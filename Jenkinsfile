node('node') {


    currentBuild.result = "SUCCESS"

    try {

       stage('Checkout'){

          checkout scm
       }

       stage('Test'){
         print "Environment will be : ${env.RUST_ENV}"

         sh 'cargo build'
       }
    }
    catch (err) {

        currentBuild.result = "FAILURE"
        throw err
    }

}

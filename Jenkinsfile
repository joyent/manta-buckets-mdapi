/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright 2020 Joyent, Inc.
 */

@Library('jenkins-joylib@v1.0.4') _

pipeline {

    agent {
        label joyCommonLabels(image_ver: '19.1.0')
    }

    options {
        buildDiscarder(logRotator(numToKeepStr: '90'))
        timestamps()
    }

    stages {
        stage('check') {
            steps{
                sh('make check')
            }
        }
        stage('test') {
            steps{
                sh('make test-unit')
            }
        }
        stage('build image and upload') {
            steps {
                joyBuildImageAndUpload()
            }
        }
    }

    post {
        always {
            joyMattermostNotification()
        }
    }
}

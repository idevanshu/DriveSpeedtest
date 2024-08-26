
<h1>Drive Speed Test</h1>

<strong>This project is a simple command-line tool written in Rust to measure the read and write speeds of mounted drives on Unix-like systems.</strong>

<h3>Overview</h3>
<strong>
    The Drive Speed Test tool allows users to select a mounted drive and test its read and write speeds by creating a 1GB test file. This is useful for benchmarking storage devices, such as SSDs, HDDs, or external drives, to assess their performance.
</strong>

<h3>Prerequisites</h3>
<ul>
    <li><strong>Rust (latest stable version)</strong></li>
    <li><strong>A Unix-like operating system (Linux, macOS)</strong></li>
    <li><strong>At least 1GB of free space on the selected drive</strong></li>
</ul>

<h3>Quickstart</h3>
<ol>
    <li><strong>Clone the repository:</strong></li>
    <pre><code>git clone https://github.com/idevanshu/DriveSpeedtest.git</code></pre>

  <li><strong>Navigate to the project directory:</strong></li>
  <pre><code>cd DriveSpeedtest</code></pre>
  <li><strong>Run the tool:</strong></li>
  <pre><code>cargo run</code></pre>
  <li><strong>Select a drive from the list to test its speed:</strong></li>
  <strong>When prompted, enter the number corresponding to the drive you wish to test. The tool will perform both write and read speed tests, displaying the results in MB/s.</strong>
</ol>

<h3>Example Output</h3>
<pre><code>
Available mounted drives:
1. /dev/sda1
2. /dev/sdb1

Select a drive to test (enter the number):
1
Write speed: 820.34 MB/s
Read speed: 1500.67 MB/s
</code></pre>

<h3>Notes</h3>
<ul>
    <li><strong>This tool currently only works on Unix-like systems as it relies on the `df` command to list mounted drives.</strong></li>
    <li><strong>Make sure you have sufficient permissions to create and delete files on the selected drive.</strong></li>
    <li><strong>The tool automatically cleans up the test file after the read test is completed.</strong></li>
</ul>



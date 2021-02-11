import alluxio
from airflow import DAG
from airflow.operators.bash_operator import BashOperator
from airflow.operators.python_operator import PythonOperator
from alluxio import option
from alluxio import wire
from datetime import datetime


def upload_to_alluxio(
    hostname, port, schema, directory, tablename, tmp_dir, source_file
):
    directory = "/" + directory + "/" + schema + "/" + tablename
    client = alluxio.Client(hostname, int(port))
    opt = option.CreateDirectory(
        recursive=True, allow_exists=True, write_type=wire.WRITE_TYPE_MUST_CACHE
    )
    client.create_directory(directory, opt)
    path = directory + "/data.csv"
    if client.exists(path):
        client.delete(path)
    opt = option.CreateFile(write_type=wire.WRITE_TYPE_MUST_CACHE)
    with client.open(path, "w", opt) as f:
        with open(tmp_dir + "/" + source_file) as source:
            try:
                f.write(source)
            except:
                pass
    print("Done uploading %s to %s" % (source_file, path))


default_args = {
    "owner": "airflow",
    "depends_on_past": False,
    "email": ["airflow@example.com"],
    "email_on_failure": False,
    "email_on_retry": False,
    "retries": 1,
    "retry_delay": 300,
}
dag = DAG(
    "flow",
    default_args=default_args,
    description="Auto-generated by Aorist",
    schedule_interval=None,
    start_date=datetime(2021, 1, 1),
    tags=["aorist"],
)
hive_created = BashOperator(
    bash_command='presto -e "{query}"'.format(
        query="CREATE SCHEMA IF NOT EXISTS {presto_schema} WITH (location='{location}')".format(
            presto_schema="snap_data",
            location="alluxio://alluxio-server:19998/data/snap_data",
        )
    ),
    dag=dag,
    task_id="hive_created",
)
table_created = BashOperator(
    bash_command='presto -e "{query}"'.format(
        query="""
CREATE TABLE IF NOT EXISTS {presto_schema}.{table_name} 
({schema}) WITH (format='{data_format}')
""".format(
            data_format="ORC",
            presto_schema="snap_data",
            schema="""
    from_id BIGINT COMMENT '',
    to_id BIGINT COMMENT ''    
""",
            table_name="ca_astroph",
        )
    ),
    dag=dag,
    task_id="table_created",
)
table_created.set_upstream([hive_created])
download_location = BashOperator(
    bash_command="mkdir -p {tmp_dir} && curl {address} -o {tmp_dir}/{file_name}".format(
        tmp_dir="/tmp/ca_astroph",
        address="https://snap.stanford.edu/data/ca-AstroPh.txt.gz",
        file_name="ca_astroph.downloaded",
    ),
    dag=dag,
    task_id="download_location",
)
decompress_gzip = BashOperator(
    bash_command="gunzip {command}".format(
        command="--suffix=downloaded -c /tmp/ca_astroph/ca_astroph.downloaded > /tmp/ca_astroph/ca_astroph.txt"
    ),
    dag=dag,
    task_id="decompress_gzip",
)
decompress_gzip.set_upstream([download_location])
remove_header = BashOperator(
    bash_command="tail -n +{n} {tmp_dir}/{file_name}.txt > {tmp_dir}/{file_name}.no_header && rm {tmp_dir}/{file_name}.txt".format(
        n="5", file_name="ca_astroph", tmp_dir="/tmp/ca_astroph"
    ),
    dag=dag,
    task_id="remove_header",
)
remove_header.set_upstream([decompress_gzip])
convert_csv = BashOperator(
    bash_command="{call}\n".format(
        call="cat /tmp/ca_astroph/ca_astroph.no_header | tr '\\t' ',' > /tmp/ca_astroph/ca_astroph.csv"
    ),
    dag=dag,
    task_id="convert_csv",
)
convert_csv.set_upstream([remove_header])
upload_alluxio = PythonOperator(
    python_callable=upload_to_alluxio, dag=dag, task_id="upload_alluxio"
)
upload_alluxio.set_upstream([table_created, convert_csv])
csv_created = BashOperator(
    bash_command='presto -e "{query}"'.format(
        query="""
CREATE TABLE IF NOT EXISTS {presto_schema}.{table_name} 
({schema})
WITH (format='CSV', external_location='{external_location}')
""".format(
            presto_schema="snap_data",
            external_location="alluxio://alluxio-server:19998/data/snap_data/ca_astroph_csv",
            schema="""    from_id VARCHAR,
    to_id VARCHAR
""",
            table_name="ca_astroph_csv",
        )
    ),
    dag=dag,
    task_id="csv_created",
)
csv_created.set_upstream([upload_alluxio])
convert_table = BashOperator(
    bash_command='presto -e "{query}"'.format(
        query="""
INSERT INTO {presto_schema}.{table_name} 
SELECT {columns} 
FROM {presto_schema}.{table_name}_csv
""".format(
            table_name="ca_astroph",
            presto_schema="snap_data",
            columns="""
CAST(from_id AS BIGINT) AS from_id,
CAST(to_id AS BIGINT) AS to_id
""",
        )
    ),
    dag=dag,
    task_id="convert_table",
)
convert_table.set_upstream([csv_created])


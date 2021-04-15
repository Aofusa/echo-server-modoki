from flask import Flask, request, jsonify


from storage import Storage


app = Flask(__name__)
app.config['JSON_AS_ASCII'] = False
storage = Storage('initialize msg')


@app.route('/', methods=['GET'])
def get_index():
    return open('../public/static/api.json').read()


@app.route('/echo', methods=['POST'])
def post_echo():
    msg = request.json['msg']
    return jsonify({'msg': msg})


@app.route('/set', methods=['POST'])
def set_msg():
    msg = request.json['msg']
    storage.set_msg(msg)
    return jsonify({'code': 0, 'result': 'success'})


@app.route('/get', methods=['GET'])
def get_msg():
    return jsonify({'msg': storage.get_msg()})


if __name__=='__main__':
    storage.start_link()
    app.run(host='127.0.0.1', port=8080, debug=True)


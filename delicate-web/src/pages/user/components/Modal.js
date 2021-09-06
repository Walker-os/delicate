import React, { PureComponent } from 'react'
import PropTypes from 'prop-types'
import { Form, Input, InputNumber, Radio, Modal, Cascader } from 'antd'
import { Trans, t } from '@lingui/macro'
import city from 'utils/city'

const FormItem = Form.Item

const formItemLayout = {
  labelCol: {
    span: 6
  },
  wrapperCol: {
    span: 14
  }
}

class UserModal extends PureComponent {
  formRef = React.createRef()

  handleOk = () => {
    const { item, onOk } = this.props
    this.formRef.current
      .validateFields()
      .then((values) => {
        onOk({ ...item, ...values })
      })
      .catch((errorInfo) => console.log(errorInfo))
  }

  render() {
    const { item = {}, onOk, form, ...modalProps } = this.props
    return (
      <Modal {...modalProps} onOk={this.handleOk}>
        <Form ref={this.formRef} name="control-ref" initialValues={{ ...item }} layout="horizontal">
          <FormItem
            name="user_name"
            rules={[{ required: true, min: 5, max: 20 }]}
            label={t`Username`}
            hasFeedback
            {...formItemLayout}
          >
            <Input />
          </FormItem>
          <FormItem name="nick_name" rules={[{ required: true }]} label={t`NickName`} hasFeedback {...formItemLayout}>
            <Input />
          </FormItem>
          <FormItem
            name="mobile"
            rules={[
              {
                required: true,
                pattern: /^1[34578]\d{9}$/
              }
            ]}
            label={t`Phone`}
            hasFeedback
            {...formItemLayout}
          >
            <Input />
          </FormItem>
          <FormItem
            name="email"
            rules={[
              {
                required: true,
                pattern: /^([a-zA-Z0-9_-])+@([a-zA-Z0-9_-])+(.[a-zA-Z0-9_-])+/,
                message: t`The input is not valid E-mail!`
              }
            ]}
            label={t`Email`}
            hasFeedback
            {...formItemLayout}
          >
            <Input />
          </FormItem>
          {modalProps.modalType !== 'update' ? (
            <FormItem
              name="certificate"
              rules={[{ required: true, min: 8, max: 20 }]}
              label={t`Password`}
              hasFeedback
              {...formItemLayout}
            >
              <Input type={'password'} />
            </FormItem>
          ) : null}
        </Form>
      </Modal>
    )
  }
}

UserModal.propTypes = {
  type: PropTypes.string,
  item: PropTypes.object,
  onOk: PropTypes.func
}

export default UserModal
